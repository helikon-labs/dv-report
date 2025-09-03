import { Constants } from '../util/constants';
import {
    Cohort,
    Delegate,
    DelegateSimilarity,
    DelegateVoteCount,
    getVoteValue,
    Network,
    Referendum,
    ReferendumStatus,
    Track,
    VoteCall,
} from './types';

const COHORT_NUMBERS = [4, 5];

interface DataStoreDelegate {}

class DataStore {
    private readonly DEFAULT_COHORT = 5;

    private delegate: DataStoreDelegate;
    private networks: Network[] = [];
    private tracks: Track[] = [];
    private referendumStatuses: ReferendumStatus[] = [];
    private delegates: Delegate[] = [];
    private referenda: Referendum[] = [];
    private selectedCohortNumber: number = this.DEFAULT_COHORT;

    private selectedNetworkIds = new Set<number>();
    private selectedStatusIds = new Set<number>();
    private selectedTrackIds = new Set<number>();

    constructor(delegate: DataStoreDelegate) {
        this.delegate = delegate;
    }

    async init() {
        this.networks = [];
        this.tracks = [];
        this.referendumStatuses = [];
        this.delegates = [];
        this.referenda = [];

        this.selectedNetworkIds.clear();
        this.selectedStatusIds.clear();
        this.selectedTrackIds.clear();
    }

    getSelectedCohortNumber(): number {
        return this.selectedCohortNumber;
    }

    async fetchNetworks() {
        this.selectedNetworkIds.clear();
        this.networks = await (
            await fetch(`${Constants.API_URL}/network`, {
                method: 'GET',
                headers: {},
            })
        ).json();
        this.networks.forEach((n) => this.selectedNetworkIds.add(n.id));
    }

    getNetworks(): Network[] {
        return this.networks;
    }

    async fetchTracks() {
        for (let i = 0; i < this.networks.length; i++) {
            this.networks[i].tracks = await (
                await fetch(`${Constants.API_URL}/network/${this.networks[i].id}/track`, {
                    method: 'GET',
                    headers: {},
                })
            ).json();
        }
        this.tracks.push(...this.networks[0].tracks);
    }

    getTracks(): Track[] {
        return this.tracks;
    }

    private async fetchCohortTracks(networkId: number, cohortNumber: number): Promise<Track[]> {
        return await (
            await fetch(`${Constants.API_URL}/network/${networkId}/cohort/${cohortNumber}/track`, {
                method: 'GET',
                headers: {},
            })
        ).json();
    }

    async fetchCohorts() {
        this.selectedTrackIds.clear();
        const cohorts: Cohort[] = await (
            await fetch(`${Constants.API_URL}/cohort`, {
                method: 'GET',
                headers: {},
            })
        ).json();
        for (const cohort of cohorts) {
            cohort.tracks = await this.fetchCohortTracks(cohort.network.id, cohort.number);
            cohort.tracks.forEach((t) => this.selectedTrackIds.add(t.id));
            const network = this.networks.find((n) => n.id == cohort.network.id)!;
            if (network.cohorts == undefined) {
                network.cohorts = [];
            }
            network.cohorts.push(cohort);
        }
    }

    async fetchReferendumStatuses() {
        this.selectedStatusIds.clear();
        this.referendumStatuses = await (
            await fetch(`${Constants.API_URL}/referendum/status`, {
                method: 'GET',
                headers: {},
            })
        ).json();
        this.referendumStatuses.forEach((s) => this.selectedStatusIds.add(s.id));
    }

    getReferendumStatuses(): ReferendumStatus[] {
        return this.referendumStatuses;
    }

    async fetchDelegates() {
        this.delegates = await (
            await fetch(`${Constants.API_URL}/delegate`, {
                method: 'GET',
                headers: {},
            })
        ).json();
        this.delegates = this.delegates.filter(
            (d) => d.delegations.findIndex((d) => d.cohortNumber == this.selectedCohortNumber) >= 0,
        );
        this.delegates.sort((d1, d2) => d1.name.localeCompare(d2.name));
    }

    getDelegates(): Delegate[] {
        return this.delegates;
    }

    async fetchNetworkCohortReferenda(networkId: number, cohortNumber: number) {
        const networkReferenda: Referendum[] = await (
            await fetch(
                `${Constants.API_URL}/network/${networkId}/cohort/${cohortNumber}/referendum`,
                {
                    method: 'GET',
                    headers: {},
                },
            )
        ).json();
        this.referenda.push(...networkReferenda);
    }

    async fetchNetworkDelegateVotes(networkId: number, delegateAccountId: string) {
        const voteCalls: VoteCall[] = await (
            await fetch(
                `${Constants.API_URL}/network/${networkId}/voter/${delegateAccountId}/vote`,
                {
                    method: 'GET',
                    headers: {},
                },
            )
        ).json();
        this.delegates
            .find(
                (delegate) =>
                    delegate.delegations.find(
                        (delegation) => delegation.delegateAccountId == delegateAccountId,
                    ) != undefined,
            )!
            .votes.push(...voteCalls);
    }

    setSelectedCohortNumber(cohortNumber: number) {
        this.selectedCohortNumber = cohortNumber;
    }

    selectNetworks(networks: Network[]) {
        this.selectedNetworkIds.clear();
        networks.forEach((n) => this.selectedNetworkIds.add(n.id));
    }

    selectTracks(tracks: Track[]) {
        this.selectedTrackIds.clear();
        tracks.forEach((t) => this.selectedTrackIds.add(t.id));
    }

    selectStatuses(statuses: ReferendumStatus[]) {
        this.selectedStatusIds.clear();
        statuses.forEach((s) => this.selectedStatusIds.add(s.id));
    }

    private getDelegateFirstVoteMap(delegate: Delegate): Map<string, VoteCall> {
        const voteMap: Map<string, VoteCall> = new Map();
        for (const vote of delegate.votes) {
            if (!this.selectedNetworkIds.has(vote.networkId)) {
                continue;
            }
            if (!vote.isSuccessful) {
                continue;
            }
            const referendum = this.referenda.find(
                (r) => r.networkId == vote.networkId && r.index == vote.referendumIndex,
            )!;
            if (!referendum) {
                // referendum is not in the given cohort - skip
                continue;
            }
            if (!this.selectedStatusIds.has(referendum.status.id)) {
                continue;
            }
            if (!this.selectedTrackIds.has(referendum.track.id)) {
                continue;
            }
            if (vote.isMultisig && !vote.isMultisigExecuted) {
                continue;
            }
            const key = `${vote.networkId}_${vote.referendumIndex}`;
            const existingVote = voteMap.get(key);
            if (existingVote) {
                if (vote.block.number == existingVote.block.number) {
                    if (vote.extrinsicIndex < existingVote.extrinsicIndex) {
                        voteMap.set(key, vote);
                    }
                } else if (vote.block.number < existingVote.block.number) {
                    voteMap.set(key, vote);
                }
            } else {
                voteMap.set(key, vote);
            }
        }
        return voteMap;
    }

    private getDelegateLastVoteMap(delegate: Delegate): Map<string, VoteCall> {
        const voteMap: Map<string, VoteCall> = new Map();
        for (const vote of delegate.votes) {
            if (!this.selectedNetworkIds.has(vote.networkId)) {
                continue;
            }
            if (!vote.isSuccessful) {
                continue;
            }
            const referendum = this.referenda.find(
                (r) => r.networkId == vote.networkId && r.index == vote.referendumIndex,
            );
            if (!referendum) {
                // referendum is not in the given cohort - skip
                continue;
            }
            if (!this.selectedStatusIds.has(referendum.status.id)) {
                continue;
            }
            if (!this.selectedTrackIds.has(referendum.track.id)) {
                continue;
            }
            if (vote.isMultisig && !vote.isMultisigExecuted) {
                continue;
            }
            const key = `${vote.networkId}_${vote.referendumIndex}`;
            const existingVote = voteMap.get(key);
            if (existingVote) {
                if (existingVote.block.number == vote.block.number) {
                    if (existingVote.extrinsicIndex < vote.extrinsicIndex) {
                        voteMap.set(key, vote);
                    }
                } else if (existingVote.block.number < vote.block.number) {
                    voteMap.set(key, vote);
                }
            } else {
                voteMap.set(key, vote);
            }
        }
        return voteMap;
    }

    getDelegateChangedVoteCount(delegate: Delegate): number {
        const filteredReferenda = this.getFilteredReferenda();
        let changedCount = 0;
        for (const referendum of filteredReferenda) {
            const referendumVotes = delegate.votes
                .filter((v) => v.networkId == referendum.networkId)
                .filter((v) => v.referendumIndex == referendum.index)
                .filter((v) => v.isSuccessful)
                .filter((v) => !v.isMultisig || v.isMultisigExecuted);
            if (referendumVotes.length == 0) {
                continue;
            }
            referendumVotes.sort((v1, v2) => {
                if (v1.block.number == v2.block.number) {
                    return v1.extrinsicIndex - v2.extrinsicIndex;
                } else {
                    return v1.block.number - v2.block.number;
                }
            });
            let currentVote = getVoteValue(referendumVotes[0]);
            for (let i = 1; i < referendumVotes.length; i++) {
                const nextVote = getVoteValue(referendumVotes[i]);
                if (currentVote != nextVote) {
                    changedCount++;
                }
                currentVote = nextVote;
            }
        }
        return changedCount;
    }

    getDelegateVoteCounts(): DelegateVoteCount[] {
        const delegateVoteCounts: DelegateVoteCount[] = [];
        const filteredReferenda = this.getFilteredReferenda();
        for (const delegate of this.delegates) {
            const delegateVoteMap = this.getDelegateLastVoteMap(delegate);
            const delegateVoteCount: DelegateVoteCount = {
                delegateId: delegate.id,
                delegateName: delegate.name,
                delegateShortName: delegate.shortName,
                ayeCount: 0,
                nayCount: 0,
                abstainCount: 0,
                missedCount: 0,
                changedCount: this.getDelegateChangedVoteCount(delegate),
            };
            for (const vote of delegateVoteMap.values()) {
                const voteValue = getVoteValue(vote);
                if (voteValue == 1) {
                    delegateVoteCount.ayeCount++;
                } else if (voteValue == -1) {
                    delegateVoteCount.nayCount++;
                } else {
                    delegateVoteCount.abstainCount++;
                }
            }
            for (const filteredReferendum of filteredReferenda) {
                const key = `${filteredReferendum.networkId}_${filteredReferendum.index}`;
                if (!delegateVoteMap.has(key)) {
                    delegateVoteCount.missedCount++;
                }
            }
            delegateVoteCounts.push(delegateVoteCount);
        }
        return delegateVoteCounts.sort((v1, v2) => {
            const v1Total = v1.nayCount + v1.abstainCount + v1.ayeCount;
            const v2Total = v2.nayCount + v2.abstainCount + v2.ayeCount;
            if (v1Total == v2Total) {
                return 0;
            } else if (v1Total < v2Total) {
                return 1;
            } else {
                return -1;
            }
        });
    }

    getDelegateSimilarities(): DelegateSimilarity[] {
        const voteMap = new Map<string, Map<string, number>>();
        for (const delegate of this.delegates) {
            const delegateVoteMap = this.getDelegateLastVoteMap(delegate);
            if (delegateVoteMap.size == 0) {
                voteMap.set(delegate.id, new Map());
                continue;
            }
            for (const vote of delegateVoteMap.values()) {
                const voteValue = getVoteValue(vote);
                if (!voteMap.has(delegate.id)) {
                    voteMap.set(delegate.id, new Map());
                }
                voteMap
                    .get(delegate.id)!
                    .set(`${vote.networkId}_${vote.referendumIndex}`, voteValue);
            }
        }
        const delegateIds = Array.from(voteMap.keys());
        const similarities: DelegateSimilarity[] = [];
        for (let i = 0; i < delegateIds.length; i++) {
            for (let j = i + 1; j < delegateIds.length; j++) {
                const aId = delegateIds[i];
                const bId = delegateIds[j];
                const aVotes = voteMap.get(aId)!;
                const bVotes = voteMap.get(bId)!;
                // find shared referenda
                const shared: string[] = [];
                for (const ref of aVotes.keys()) {
                    if (!bVotes.has(ref)) continue;
                    const a = aVotes.get(ref)!;
                    const b = bVotes.get(ref)!;
                    if (a === 0 || b === 0) continue; // skip abstains
                    shared.push(ref);
                }
                if (shared.length === 0) {
                    similarities.push({ aId: aId, bId: bId, value: 0 });
                    continue;
                }
                // mean agreement: average of (a.vote === b.vote)
                const scoreSum = shared.reduce((sum, ref) => {
                    const a = aVotes.get(ref)!;
                    const b = bVotes.get(ref)!;
                    return sum + (a === b ? 1 : -1);
                }, 0);
                const similarity = scoreSum / shared.length;
                similarities.push({ aId: aId, bId: bId, value: similarity });
            }
        }
        return similarities;
    }

    getResponseTimes(): Map<Delegate, number> {
        const responseTimeMap = new Map<Delegate, number>();
        for (const delegate of this.delegates) {
            const delegateVoteMap = this.getDelegateFirstVoteMap(delegate);
            let responseTimeSum = 0;
            for (const vote of delegateVoteMap.values()) {
                const referendum = this.referenda.find(
                    (r) => r.networkId == vote.networkId && r.index == vote.referendumIndex,
                )!;
                responseTimeSum += vote.block.number - referendum.submissionBlock.number;
            }
            if (delegateVoteMap.size == 0) {
                responseTimeMap.set(delegate, 0);
            } else {
                const averageResponseTime = responseTimeSum / delegateVoteMap.size;
                responseTimeMap.set(delegate, Math.floor(averageResponseTime));
            }
        }
        return responseTimeMap;
    }

    getFilteredReferenda(): Referendum[] {
        const referenda: Referendum[] = [];
        for (const referendum of this.referenda) {
            if (!this.selectedNetworkIds.has(referendum.networkId)) {
                continue;
            }
            if (!this.selectedStatusIds.has(referendum.status.id)) {
                continue;
            }
            if (!this.selectedTrackIds.has(referendum.track.id)) {
                continue;
            }
            referenda.push(referendum);
        }
        referenda.sort((r1, r2) => r1.index - r2.index);
        return referenda;
    }

    getAllDelegatesLastVoteMaps(): Map<string, Map<string, VoteCall>> {
        const map: Map<string, Map<string, VoteCall>> = new Map<string, Map<string, VoteCall>>();
        for (const delegate of this.delegates) {
            map.set(delegate.id, this.getDelegateLastVoteMap(delegate));
        }
        return map;
    }
}

export { COHORT_NUMBERS, DataStore, DataStoreDelegate };
