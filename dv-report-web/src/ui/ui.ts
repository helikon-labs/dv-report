import * as d3 from 'd3';
import { show, hide } from '../util/ui-util';
import {
    Delegate,
    DelegateSimilarity,
    DelegateVoteCount,
    Network,
    ReferendumStatus,
    Track,
} from '../data/types';

interface UIDelegate {
    onNetworkSelectChanged(value: string): void;
    onTrackSelectChanged(value: string): void;
    onStatusSelectChanged(value: string): void;
}

class UI {
    private readonly root: HTMLDivElement;
    private readonly content: HTMLDivElement;
    private readonly loadingContainer: HTMLDivElement;
    private readonly loadingDescription: HTMLDivElement;

    private readonly filterContainer: HTMLDivElement;
    private readonly networkSelect: HTMLSelectElement;
    private readonly trackSelect: HTMLSelectElement;
    private readonly statusSelect: HTMLSelectElement;

    private delegate: UIDelegate;
    private voteCountsMaxX = 0;
    private policyMaxX = 0;

    private readonly nayColor = '#f44336';
    private readonly abstainColor = '#aaaaaa';
    private readonly ayeColor = '#4caf50';
    private similarityGroup: d3.Selection<SVGGElement, unknown, HTMLElement, any> | null = null;

    constructor(delegate: UIDelegate) {
        this.delegate = delegate;
        this.root = <HTMLDivElement>document.getElementById('root');
        this.content = <HTMLDivElement>document.getElementById('content');
        this.loadingContainer = <HTMLDivElement>document.getElementById('loading-container');
        this.loadingDescription = <HTMLDivElement>document.getElementById('loading-description');

        this.filterContainer = <HTMLDivElement>document.getElementById('filter-container');
        this.networkSelect = <HTMLSelectElement>document.getElementById('network-select');
        this.trackSelect = <HTMLSelectElement>document.getElementById('track-select');
        this.statusSelect = <HTMLSelectElement>document.getElementById('status-select');
    }

    lock() {
        hide(this.filterContainer);
        hide(this.content);
        show(this.loadingContainer);
    }

    unlock() {
        show(this.filterContainer);
        show(this.content);
        hide(this.loadingContainer);
    }

    setLoadingDescription(description: string) {
        this.loadingDescription.innerHTML = description;
    }

    initFilters(networks: Network[], tracks: Track[], statuses: ReferendumStatus[]) {
        let networkSelectHTML = '<option value="all" selected>All</option>';
        networks.forEach((n) => {
            networkSelectHTML += `<option value="${n.id}">${n.display}</option>`;
        });
        this.networkSelect.innerHTML = networkSelectHTML;
        this.networkSelect.onchange = (_) => {
            this.delegate.onNetworkSelectChanged(this.networkSelect.value);
        };

        let trackSelectHTML = '<option value="all">All</option>';
        trackSelectHTML += '<option value="dv" selected>DV</option>';
        for (const track of tracks) {
            trackSelectHTML += `<option value="${track.id}">${track.name}</option>`;
        }
        this.trackSelect.innerHTML = trackSelectHTML;
        this.trackSelect.onchange = (_) => {
            this.delegate.onTrackSelectChanged(this.trackSelect.value);
        };

        let statusSelectHTML = '<option value="all">All</option>';
        for (const status of statuses) {
            statusSelectHTML += `<option value="${status.id}">${status.status}</option>`;
        }
        this.statusSelect.innerHTML = statusSelectHTML;
        this.statusSelect.onchange = (_) => {
            this.delegate.onStatusSelectChanged(this.statusSelect.value);
        };
    }

    displayVoteCountChart(data: DelegateVoteCount[]) {
        type StackedDatum = d3.SeriesPoint<DelegateVoteCount> & { key: keyof DelegateVoteCount };
        const totals = data.map((d) => ({
            delegateName: d.delegateName,
            delegateId: d.delegateId,
            total: d.nayCount + d.abstainCount + d.ayeCount,
        }));
        const stackKeys = ['nayCount', 'abstainCount', 'ayeCount'] as const;
        const color = d3
            .scaleOrdinal<string>()
            .domain(stackKeys)
            .range([this.nayColor, this.abstainColor, this.ayeColor]);

        const width = 800;
        const height = 320;
        const margin = { top: 12, right: 20, bottom: 16, left: 120 };

        const svg = d3
            .select<SVGSVGElement, unknown>('#vote-count-chart')
            .attr('viewBox', `0 0 ${width} ${height}`);

        // Fixed max x-domain for smooth updates
        const newMax = d3.max(data, (d) => stackKeys.reduce((sum, key) => sum + d[key], 0))!;
        this.voteCountsMaxX = Math.max(this.voteCountsMaxX, newMax);

        const x = d3
            .scaleLinear()
            .domain([0, this.voteCountsMaxX + 5])
            .range([margin.left, width - margin.right]);
        const y = d3
            .scaleBand()
            .domain(data.map((d) => d.delegateName))
            .range([margin.top, height - margin.bottom])
            .padding(0.1);
        const stackedData = d3.stack<DelegateVoteCount>().keys(stackKeys)(data);
        // bars
        const barGroups = svg
            .selectAll<SVGGElement, d3.Series<DelegateVoteCount, string>>('g.layer')
            .data(stackedData, (d: any) => d.key);
        const barGroupsEnter = barGroups
            .enter()
            .append('g')
            .attr('class', 'layer')
            .attr('fill', (d) => color(d.key)!);
        barGroupsEnter
            .merge(barGroups)
            .selectAll<SVGRectElement, StackedDatum>('rect')
            .data(
                (d) =>
                    d.map((point) =>
                        Object.assign(point, { key: d.key as keyof DelegateVoteCount }),
                    ),
                (d) => d.data.delegateId + '-' + d.key,
            )
            .join(
                (enter) =>
                    enter
                        .append('rect')
                        .attr('x', (d) => x(d[0]))
                        .attr('y', (d) => y(d.data.delegateName)!)
                        .attr('width', (d) => x(d[1]) - x(d[0]))
                        .attr('height', y.bandwidth()),
                (update) =>
                    update
                        .transition()
                        .duration(750)
                        .attr('x', (d) => x(d[0]))
                        .attr('width', (d) => x(d[1]) - x(d[0]))
                        .attr('y', (d) => y(d.data.delegateName)!)
                        .attr('height', y.bandwidth()),
                (exit) => exit.remove(),
            );
        // labels
        barGroupsEnter
            .merge(barGroups)
            .selectAll<SVGTextElement, StackedDatum>('text')
            .data(
                (d) =>
                    d.map((point) =>
                        Object.assign(point, { key: d.key as keyof DelegateVoteCount }),
                    ),
                (d) => d.data.delegateId + '-' + d.key,
            )
            .join(
                (enter) =>
                    enter
                        .append('text')
                        .attr('text-anchor', 'middle')
                        .attr('dy', '0.35em')
                        .style('fill', 'white')
                        .style('font-size', '11px')
                        .attr('x', (d) => x(d[0]) + (x(d[1]) - x(d[0])) / 2)
                        .attr('y', (d) => y(d.data.delegateName)! + y.bandwidth() / 2)
                        .text((d) => {
                            const w = x(d[1]) - x(d[0]);
                            return w > 10 ? String(d.data[d.key]) : '';
                        }),

                (update) =>
                    update
                        .transition()
                        .duration(750)
                        .attr('x', (d) => x(d[0]) + (x(d[1]) - x(d[0])) / 2)
                        .attr('y', (d) => y(d.data.delegateName)! + y.bandwidth() / 2)
                        .text((d) => {
                            const w = x(d[1]) - x(d[0]);
                            return w > 10 ? String(d.data[d.key]) : '';
                        }),

                (exit) => exit.remove(),
            );
        // total labels at the end of each stacked bar
        svg.selectAll<SVGTextElement, (typeof totals)[0]>('.total-label')
            .data(totals, (d) => d.delegateId)
            .join(
                (enter) =>
                    enter
                        .append('text')
                        .attr('class', 'total-label')
                        .attr('x', (d) => x(d.total) + 4) // 4px padding after bar
                        .attr('y', (d) => y(d.delegateName)! + y.bandwidth() / 2)
                        .attr('dy', '0.35em')
                        .attr('text-anchor', 'start')
                        .style('fill', 'black')
                        .style('font-size', '11px')
                        .text((d) => d.total),
                (update) =>
                    update
                        .transition()
                        .duration(750)
                        .attr('x', (d) => x(d.total) + 4)
                        .attr('y', (d) => y(d.delegateName)! + y.bandwidth() / 2)
                        .text((d) => d.total),
                (exit) => exit.remove(),
            );

        // axes
        svg.selectAll('.x-axis')
            .data([null])
            .join(
                (enter) =>
                    enter
                        .append('g')
                        .attr('class', 'x-axis')
                        .attr('transform', `translate(0,${height - margin.bottom})`)
                        .call(d3.axisBottom(x)),
                (update) =>
                    update
                        .transition()
                        .duration(750)
                        // @ts-ignore
                        .call(d3.axisBottom(x)),
            );
        svg.selectAll('.y-axis')
            .data([null])
            .join(
                (enter) =>
                    enter
                        .append('g')
                        .attr('class', 'y-axis')
                        .attr('transform', `translate(${margin.left},0)`)
                        .call(d3.axisLeft(y)),
                (update) =>
                    update
                        .transition()
                        .duration(750)
                        // @ts-ignore
                        .call(d3.axisLeft(y)),
            );
        // cleanup exit
        barGroups.exit().remove();
    }

    displayPolicyDirectionChart(data: DelegateVoteCount[]) {
        const width = 800;
        const height = 320;
        const margin = { top: 12, right: 20, bottom: 16, left: 120 };

        const svg = d3
            .select<SVGSVGElement, unknown>('#policy-direction-chart')
            .attr('viewBox', `0 0 ${width} ${height}`);

        // Compute and sort
        const scoredData = data.map((d) => ({
            ...d,
            score: d.ayeCount - d.nayCount,
        }));

        scoredData.sort((a, b) => b.score - a.score);

        // Cache max absolute score
        const maxScore = d3.max(scoredData, (d) => Math.abs(d.score))!;
        this.policyMaxX = Math.max(this.policyMaxX, maxScore);

        const x = d3
            .scaleLinear()
            .domain([-this.policyMaxX, this.policyMaxX])
            .range([margin.left, width - margin.right]);

        const y = d3
            .scaleBand()
            .domain(scoredData.map((d) => d.delegateName))
            .range([margin.top, height - margin.bottom])
            .padding(0.1);

        // Update axes
        const xAxis = svg.selectAll<SVGGElement, unknown>('.x-axis').data([null]);

        xAxis.join(
            (enter) =>
                enter
                    .append('g')
                    .attr('class', 'x-axis')
                    .attr('transform', `translate(0,${height - margin.bottom})`)
                    .call(d3.axisBottom(x).ticks(5)),
            (update) => update.transition().duration(750).call(d3.axisBottom(x).ticks(5)),
        );

        const yAxis = svg.selectAll<SVGGElement, unknown>('.y-axis').data([null]);

        yAxis.join(
            (enter) =>
                enter
                    .append('g')
                    .attr('class', 'y-axis')
                    .attr('transform', `translate(${margin.left},0)`)
                    .call(d3.axisLeft(y).tickSize(0)),
            (update) => update.transition().duration(750).call(d3.axisLeft(y).tickSize(0)),
        );

        // bars
        const bars = svg
            .selectAll<SVGRectElement, (typeof scoredData)[0]>('.bar')
            .data(scoredData, (d) => d.delegateId);
        bars.join(
            (enter) =>
                enter
                    .append('rect')
                    .attr('class', 'bar')
                    .attr('x', (d) =>
                        d.score === 0
                            ? x(0) - 1 // small bar width, centered on 0
                            : x(Math.min(0, d.score)),
                    )
                    .attr('y', (d) => y(d.delegateName)!)
                    .attr('width', (d) => (d.score === 0 ? 2 : Math.abs(x(d.score) - x(0))))
                    .attr('height', y.bandwidth())
                    .attr('fill', (d) =>
                        d.score > 0
                            ? this.ayeColor
                            : d.score < 0
                              ? this.nayColor
                              : this.abstainColor,
                    ),

            (update) =>
                update
                    .transition()
                    .duration(750)
                    .attr('x', (d) => (d.score === 0 ? x(0) - 1 : x(Math.min(0, d.score))))
                    .attr('width', (d) => (d.score === 0 ? 2 : Math.abs(x(d.score) - x(0))))
                    .attr('y', (d) => y(d.delegateName)!)
                    .attr('height', y.bandwidth())
                    .attr('fill', (d) =>
                        d.score > 0
                            ? this.ayeColor
                            : d.score < 0
                              ? this.nayColor
                              : this.abstainColor,
                    ),

            (exit) => exit.remove(),
        );

        const labels = svg
            .selectAll<SVGTextElement, (typeof scoredData)[0]>('.bar-label')
            .data(scoredData, (d) => d.delegateId);
        // enter selection (no transition)
        labels
            .enter()
            .append('text')
            .attr('class', 'bar-label')
            .style('fill', 'white')
            .style('font-size', '11px')
            .attr('text-anchor', 'middle')
            .attr('dy', '0.35em')
            .attr('x', (d) => {
                const barStart = x(Math.min(0, d.score));
                const barEnd = x(Math.max(0, d.score));
                const barWidth = Math.abs(barEnd - barStart);
                return barStart + barWidth / 2;
            })
            .attr('y', (d) => y(d.delegateName)! + y.bandwidth() / 2)
            .text((d) => {
                const barStart = x(Math.min(0, d.score));
                const barEnd = x(Math.max(0, d.score));
                const barWidth = Math.abs(barEnd - barStart);
                const plusSign = d.score > 0 ? '+' : '';
                return barWidth > 10 ? `${plusSign}${d.score}` : '';
            });
        // update selection (with transition)
        labels
            .transition()
            .duration(750)
            .attr('x', (d) => {
                const barStart = x(Math.min(0, d.score));
                const barEnd = x(Math.max(0, d.score));
                const barWidth = Math.abs(barEnd - barStart);
                return barStart + barWidth / 2;
            })
            .attr('y', (d) => y(d.delegateName)! + y.bandwidth() / 2)
            .text((d) => {
                const barStart = x(Math.min(0, d.score));
                const barEnd = x(Math.max(0, d.score));
                const barWidth = Math.abs(barEnd - barStart);
                const plusSign = d.score > 0 ? '+' : '';
                return barWidth > 10 ? `${plusSign}${d.score}` : '';
            });
    }

    displaySimilarityMatrixChart(delegates: Delegate[], similarities: DelegateSimilarity[]) {
        const cellWidth = 112;
        const cellHeight = 42;
        const margin = { top: 50, left: 90, bottom: 20, right: 20 };
        const width = (delegates.length - 1) * cellWidth + margin.left + margin.right;
        const height = (delegates.length - 1) * cellHeight + margin.top + margin.bottom;

        const svg = d3
            .select<SVGSVGElement, unknown>('#similarity-matrix-chart')
            .attr('viewBox', `0 0 ${width} ${height}`);

        const color = d3
            .scaleLinear<string>()
            .domain([-1, 0, 1])
            .range([this.nayColor, this.abstainColor, this.ayeColor]);

        const radius = d3
            .scaleSqrt()
            .domain([0, 1])
            .range([0, Math.min(cellWidth, cellHeight) / 2 - 1]);

        if (svg.select('.grid-lines').empty()) {
            // Hhrizontal grid lines
            svg.append('g')
                .attr('class', 'grid-lines horizontal')
                .attr('stroke', '#dddddd')
                .attr('stroke-width', 0.5)
                .selectAll('line')
                .data(d3.range(delegates.length - 1))
                .join('line')
                .attr('x1', margin.left)
                .attr('x2', margin.left + (delegates.length - 1) * cellWidth)
                .attr('y1', (d) => margin.top + d * cellHeight + cellHeight / 2)
                .attr('y2', (d) => margin.top + d * cellHeight + cellHeight / 2);

            // vertical grid lines
            svg.append('g')
                .attr('class', 'grid-lines vertical')
                .attr('stroke', '#dddddd')
                .attr('stroke-width', 0.5)
                .selectAll('line')
                .data(d3.range(delegates.length - 1))
                .join('line')
                .attr('y1', margin.top)
                .attr('y2', margin.top + (delegates.length - 1) * cellHeight)
                .attr('x1', (d) => margin.left + d * cellWidth + cellWidth / 2)
                .attr('x2', (d) => margin.left + d * cellWidth + cellWidth / 2);

            // row labels
            svg.append('g')
                .selectAll('text.row-label')
                .data(delegates.slice(0, delegates.length - 1))
                .join('text')
                .attr('class', 'row-label')
                .attr('x', margin.left - 10)
                .attr('y', (_, i) => margin.top + i * cellHeight + cellHeight / 2)
                .attr('dy', '0.35em')
                .attr('text-anchor', 'end')
                .style('font-size', '9px')
                .style('font-family', 'Inter')
                .text((d) => d.shortName);

            // column labels
            svg.append('g')
                .selectAll('text.column-label')
                .data(delegates.slice(1, delegates.length))
                .join('text')
                .attr('class', 'column-label')
                .attr(
                    'x',
                    (_, i) => margin.left + (delegates.length - 2 - i) * cellWidth + cellWidth / 2,
                )
                .attr('y', margin.top - 12)
                .attr('text-anchor', 'middle')
                .style('font-size', '9px')
                .style('font-family', 'Inter')
                .text((d) => d.shortName);

            // Create the similarity group container
            this.similarityGroup = svg
                .append('g')
                .attr('class', 'similarity-group')
                .attr('transform', `translate(${margin.left},${margin.top})`);
        }

        // Compute similarity pairs
        const pairs: { a: Delegate; b: Delegate; value: number; row: number; col: number }[] = [];
        for (let i = 0; i < delegates.length; i++) {
            for (let j = i; j < delegates.length; j++) {
                if (i === j) continue;
                const a = delegates[i];
                const b = delegates[j];
                const similarity = similarities.find(
                    (s) => (s.aId == a.id && s.bId == b.id) || (s.aId == b.id && s.bId == a.id),
                )!;
                const sim = similarity.value;
                //pairs.push({ a, b, value: sim, row: i, col: j });
                pairs.push({ a, b, value: sim, row: i, col: delegates.length - 1 - j });
            }
        }

        // Animate similarity circles
        this.similarityGroup!.selectAll<SVGCircleElement, (typeof pairs)[0]>('circle')
            .data(pairs, (d) => `${d.a.id}-${d.b.id}`)
            .join(
                (enter) =>
                    enter
                        .append('circle')
                        .attr('cx', (d) => d.col * cellWidth + cellWidth / 2)
                        .attr('cy', (d) => d.row * cellHeight + cellHeight / 2)
                        .attr('r', (d) => radius(Math.abs(d.value)))
                        .attr('fill', (d) => color(d.value)),
                (update) =>
                    update
                        .transition()
                        .duration(750)
                        .attr('cx', (d) => d.col * cellWidth + cellWidth / 2)
                        .attr('cy', (d) => d.row * cellHeight + cellHeight / 2)
                        .attr('r', (d) => radius(Math.abs(d.value)))
                        .attr('fill', (d) => color(d.value)),
                (exit) => exit.remove(),
            );
    }
}

export { UI, UIDelegate };
