import * as d3 from 'd3';
import { show, hide } from '../util/ui-util';
import { DelegateVoteCount } from '../data/types';

interface UIDelegate {
    onStuff(): void;
}

class UI {
    private readonly root: HTMLDivElement;
    private readonly content: HTMLDivElement;
    private readonly loadingContainer: HTMLDivElement;
    private readonly loadingDescription: HTMLDivElement;

    private delegate: UIDelegate;

    constructor(delegate: UIDelegate) {
        this.delegate = delegate;
        this.root = <HTMLDivElement>document.getElementById('root');
        this.content = <HTMLDivElement>document.getElementById('content');
        this.loadingContainer = <HTMLDivElement>document.getElementById('loading-container');
        this.loadingDescription = <HTMLDivElement>document.getElementById('loading-description');
    }

    lock() {
        hide(this.content);
        show(this.loadingContainer);
    }

    unlock() {
        show(this.content);
        hide(this.loadingContainer);
    }

    setLoadingDescription(description: string) {
        this.loadingDescription.innerHTML = description;
    }

    barChart(data: DelegateVoteCount[]) {
        type StackedDatum = d3.SeriesPoint<DelegateVoteCount> & { key: keyof DelegateVoteCount };
        const stackKeys = ['nayCount', 'abstainCount', 'ayeCount'] as const;
        const color = d3.scaleOrdinal<string>()
            .domain(stackKeys)
            .range(['#f44336', '#ffca28', '#4caf50']);

        const width = 800;
        const height = 360;
        const margin = { top: 20, right: 20, bottom: 20, left: 120 };

        const svg = d3
            .select<SVGSVGElement, unknown>('#chart')
            .attr('viewBox', `0 0 ${width} ${height}`);
        svg.selectAll('*').remove();

        const x = d3.scaleLinear()
            .domain([
                0,
                d3.max(data, d => stackKeys.reduce((sum, key) => sum + d[key], 0))!
            ])
            .nice()
            .range([margin.left, width - margin.right]);
        const y = d3.scaleBand()
            .domain(data.map(d => d.delegateName))
            .range([margin.top, height - margin.bottom])
            .padding(0.1);
        const stackedData = d3.stack<DelegateVoteCount>()
            .keys(stackKeys)
            (data);
        const barGroups = svg.append('g')
            .selectAll('g')
            .data(stackedData)
            .join('g')
            .attr('fill', d => color(d.key)!);

        barGroups
            .selectAll<SVGRectElement, StackedDatum>('rect')
            .data(d => d.map(point => Object.assign(point, { key: d.key as keyof DelegateVoteCount })))
            .join('rect')
            .attr('x', d => x(d[0]))
            .attr('y', d => y(d.data.delegateName)!)
            .attr('width', d => x(d[1]) - x(d[0]))
            .attr('height', y.bandwidth());
        barGroups
            .selectAll('text')
            .data(d => d.map(point => Object.assign(point, { key: d.key as keyof DelegateVoteCount })))
            .join('text')
            .attr('x', d => x(d[0]) + (x(d[1]) - x(d[0])) / 2)
            .attr('y', d => y(d.data.delegateName)! + y.bandwidth() / 2)
            .attr('text-anchor', 'middle')
            .attr('dy', '0.35em')
            .style('fill', 'white')
            .style('font-size', '11px')
            .text(d => {
                const w = x(d[1]) - x(d[0]);
                return w > 20 ? String(d.data[d.key]) : '';
            });

        svg.append('g').attr('transform', `translate(0,${height - margin.bottom})`).call(d3.axisBottom(x));
        svg.append('g').attr('transform', `translate(${margin.left},0)`).call(d3.axisLeft(y));
    }
}

export { UI, UIDelegate };
