import { Constants } from '../util/constants';
import { Cohort, Delegate, DelegateVoteCount, Network, Referendum, ReferendumStatus, Track, VoteCall } from './types';

interface DataStoreDelegate {}

class DataStore {
    private delegate: DataStoreDelegate;
    private networks: Network[] = [];
    private tracks: Track[] = [];
    private cohorts: Cohort[] = [];
    private referendumStatuses: ReferendumStatus[] = [];
    private delegates: Delegate[] = [];
    private referenda: Referendum[] = [];

    private selectedNetworks: Network[] = [];
    private selectedStatuses: ReferendumStatus[] = [];
    private selectedTracks: Track[] = [];

    constructor(delegate: DataStoreDelegate) {
        this.delegate = delegate;
    }

    async init() {}

    async fetchNetworks() {
        this.networks = await (
            await fetch(`${Constants.API_URL}/network`, {
                method: 'GET',
                headers: {},
            })
        ).json();
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
        this.cohorts = await (
            await fetch(`${Constants.API_URL}/cohort`, {
                method: 'GET',
                headers: {},
            })
        ).json();
        for (let i = 0; i < this.cohorts.length; i++) {
            this.cohorts[i].tracks = await this.fetchCohortTracks(
                this.cohorts[i].network.id,
                this.cohorts[i].number,
            );
        }
    }

    getCohorts(): Cohort[] {
        return this.cohorts;
    }

    async fetchReferendumStatuses() {
        this.referendumStatuses = await (
            await fetch(`${Constants.API_URL}/referendum/status`, {
                method: 'GET',
                headers: {},
            })
        ).json();
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
        let networkReferenda: Referendum[] = await (
            await fetch(`${Constants.API_URL}/network/${networkId}/referendum`, {
                method: 'GET',
                headers: {},
            })
        ).json();
        this.referenda.push(...networkReferenda);
    }

    async fetchNetworkDelegateVotes(networkId: number, delegateAccountId: string) {
        let voteCalls: VoteCall[] = await (
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

    getDelegateVoteCounts(): DelegateVoteCount[] {
        let delegateVoteCounts: DelegateVoteCount[] = [];
        for (let delegate of this.delegates) {
            let voteMap: Map<String, VoteCall> = new Map();
            for (let vote of delegate.votes) {
                if (vote.networkId != 1) {
                    continue;
                }
                let key = `${vote.networkId}_${vote.referendumIndex}`;
                let existingVote = voteMap.get(key);
                if (existingVote) {
                    if (existingVote.block.number == vote.block.number) {
                        if (existingVote.extrinsicIndex < vote.extrinsicIndex) {
                            voteMap.set(key, vote);
                        }
                    }
                    if (existingVote.block.number < vote.block.number) {
                        voteMap.set(key, vote);
                    }
                } else {
                    voteMap.set(key, vote);
                }
            }
            let delegateVoteCount: DelegateVoteCount = {
                delegateId: delegate.id,
                delegateName: delegate.name,
                ayeCount: 0,
                nayCount: 0,
                abstainCount: 0,
            }
            console.log(delegate.name, voteMap.size, 'votes');
            for (let vote of voteMap.values()) {
                switch (vote.voteType) {
                    case 'standard': {
                        if (vote.isAye!) {
                            delegateVoteCount.ayeCount++;
                        } else {
                            delegateVoteCount.nayCount++;
                        }
                        break;
                    }
                    case 'split': {
                        console.log('HMMMMM', delegate.name);
                        break;
                    }
                    case 'split_abstain': {
                        delegateVoteCount.abstainCount++;
                        break;
                    }
                    default: {
                        console.log('unknown');
                        break;
                    }
                }
            }
            delegateVoteCounts.push(delegateVoteCount);
        }
        return delegateVoteCounts.sort((v1, v2) => {
            let v1Total = v1.nayCount + v1.abstainCount + v1.ayeCount;
            let v2Total = v2.nayCount + v2.abstainCount + v2.ayeCount;
            if (v1Total == v2Total) {
                return 0;
            } else if (v1Total < v2Total) {
                return 1;
            } else {
                return -1;
            }
        });
    }
}

export { DataStore, DataStoreDelegate };
