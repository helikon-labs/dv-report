import { Constants } from '../util/constants';
import {
    Cohort,
    Delegate,
    DelegateSimilarity,
    DelegateVoteCount,
    Network,
    Referendum,
    ReferendumStatus,
    Track,
    VoteCall,
} from './types';

interface DataStoreDelegate {}

class DataStore {
    private delegate: DataStoreDelegate;
    private networks: Network[] = [];
    private tracks: Track[] = [];
    private referendumStatuses: ReferendumStatus[] = [];
    private delegates: Delegate[] = [];
    private referenda: Referendum[] = [];

    private selectedNetworkIds = new Set<number>();
    private selectedStatusIds = new Set<number>();
    private selectedTrackIds = new Set<number>();

    constructor(delegate: DataStoreDelegate) {
        this.delegate = delegate;
    }

    async init() {}

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
    }

    getDelegates(): Delegate[] {
        return this.delegates;
    }

    async fetchNetworkReferenda(networkId: number) {
        const networkReferenda: Referendum[] = await (
            await fetch(`${Constants.API_URL}/network/${networkId}/referendum`, {
                method: 'GET',
                headers: {},
            })
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

    private getDelegateVoteMap(delegate: Delegate): Map<string, VoteCall> {
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
            if (!this.selectedStatusIds.has(referendum.statusId)) {
                continue;
            }
            if (!this.selectedTrackIds.has(referendum.trackId)) {
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

    private getVoteValue(vote: VoteCall): number {
        switch (vote.voteType) {
            case 'standard': {
                if (vote.isAye!) {
                    return 1;
                } else {
                    return -1;
                }
            }
            default: {
                return 0;
            }
        }
    }

    getDelegateVoteCounts(): DelegateVoteCount[] {
        const delegateVoteCounts: DelegateVoteCount[] = [];
        for (const delegate of this.delegates) {
            const delegateVoteMap = this.getDelegateVoteMap(delegate);
            const delegateVoteCount: DelegateVoteCount = {
                delegateId: delegate.id,
                delegateName: delegate.name,
                ayeCount: 0,
                nayCount: 0,
                abstainCount: 0,
            };
            for (const vote of delegateVoteMap.values()) {
                const voteValue = this.getVoteValue(vote);
                if (voteValue == 1) {
                    delegateVoteCount.ayeCount++;
                } else if (voteValue == -1) {
                    delegateVoteCount.nayCount++;
                } else {
                    delegateVoteCount.abstainCount++;
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
            const delegateVoteMap = this.getDelegateVoteMap(delegate);
            for (const vote of delegateVoteMap.values()) {
                const voteValue = this.getVoteValue(vote);
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
                if (shared.length === 0) continue;
                // mean agreement: average of (a.vote === b.vote)
                const scoreSum = shared.reduce((sum, ref) => {
                    const a = aVotes.get(ref)!;
                    const b = bVotes.get(ref)!;
                    console.log(ref, aId, a, bId, b);
                    return sum + (a === b ? 1 : -1);
                }, 0);
                const similarity = scoreSum / shared.length;
                similarities.push({ aId: aId, bId: bId, value: similarity });
            }
        }
        return similarities;
    }
}

export { DataStore, DataStoreDelegate };
