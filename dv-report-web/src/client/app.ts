import { DataStore, DataStoreDelegate } from '../data/data-store';
import { UI, UIDelegate } from '../ui/ui';
import { sleep } from '../util/async-util';
import { Constants } from '../util/constants';

class App {
    private readonly ui: UI;
    private readonly uiDelegate: UIDelegate = {
        onNetworkSelectChanged: (value) => {
            if (value == 'all') {
                this.dataStore.selectNetworks(this.dataStore.getNetworks());
            } else {
                const network = this.dataStore.getNetworks().find((n) => n.id.toString() == value);
                if (network) {
                    this.dataStore.selectNetworks([network]);
                }
            }
            this.updateVoteCounts();
        },
        onTrackSelectChanged: (value) => {
            if (value == 'all') {
                this.dataStore.selectTracks(this.dataStore.getTracks());
            } else if (value == 'dv') {
                const tracks = this.dataStore.getNetworks()[0].cohorts[0].tracks;
                this.dataStore.selectTracks(tracks);
            } else {
                const track = this.dataStore.getTracks().find((t) => t.id.toString() == value)!;
                this.dataStore.selectTracks([track]);
            }
            this.updateVoteCounts();
        },
        onStatusSelectChanged: (value) => {
            if (value == 'all') {
                this.dataStore.selectStatuses(this.dataStore.getReferendumStatuses());
            } else {
                const status = this.dataStore
                    .getReferendumStatuses()
                    .find((s) => s.id.toString() == value)!;
                this.dataStore.selectStatuses([status]);
            }
            this.updateVoteCounts();
        },
    };

    private readonly dataStore: DataStore;
    private readonly dataStoreDelegate = <DataStoreDelegate>{};

    constructor() {
        this.dataStore = new DataStore(this.dataStoreDelegate);
        this.ui = new UI(this.uiDelegate);
    }

    async start() {
        this.ui.lock();
        try {
            await this.initData();
            this.ui.initFilters(
                this.dataStore.getNetworks(),
                this.dataStore.getTracks(),
                this.dataStore.getReferendumStatuses(),
            );
            this.updateVoteCounts();
        } catch (error) {
            alert(`Error while fetching initial data: ${error}. Please reload the page.`);
            return;
        }
        this.ui.unlock();
    }

    private updateVoteCounts() {
        const voteCountData = this.dataStore.getDelegateVoteCounts();
        this.ui.displayVoteCountChart(voteCountData);
        this.ui.displayPolicyDirectionChart(voteCountData);
        const delegates = this.dataStore.getDelegates();
        const similarities = this.dataStore.getDelegateSimilarities();
        this.ui.displaySimilarityMatrixChart(delegates, similarities);
        this.ui.displayFirstVoteTimeChart(this.dataStore.getResponseTimes());
        this.ui.displayMissedVoteCountChart(voteCountData);
        this.ui.displayChangedVoteCountChart(voteCountData);
        const referenda = this.dataStore.getFilteredReferenda();
        const lastVotesMaps = this.dataStore.getAllDelegatesLastVoteMaps();
        this.ui.displayVoteList(this.dataStore.getNetworks(), delegates, referenda, lastVotesMaps);
    }

    private async initData() {
        this.ui.setLoadingDescription('loading networks');
        await Promise.all([
            this.dataStore.fetchNetworks(),
            sleep(Constants.LOADING_STATE_TRANSITION_MIN_MS),
        ]);

        this.ui.setLoadingDescription('loading tracks');
        await Promise.all([
            this.dataStore.fetchTracks(),
            sleep(Constants.LOADING_STATE_TRANSITION_MIN_MS),
        ]);
        await sleep(500);

        this.ui.setLoadingDescription('loading cohorts');
        await Promise.all([
            this.dataStore.fetchCohorts(),
            sleep(Constants.LOADING_STATE_TRANSITION_MIN_MS),
        ]);
        await sleep(500);

        this.ui.setLoadingDescription('loading statuses');
        await Promise.all([
            this.dataStore.fetchReferendumStatuses(),
            sleep(Constants.LOADING_STATE_TRANSITION_MIN_MS),
        ]);
        await sleep(500);

        this.ui.setLoadingDescription('loading delegates');
        await Promise.all([
            this.dataStore.fetchDelegates(),
            sleep(Constants.LOADING_STATE_TRANSITION_MIN_MS),
        ]);

        for (const network of this.dataStore.getNetworks()) {
            this.ui.setLoadingDescription(`loading<br>${network.display} referenda`);
            await Promise.all([
                this.dataStore.fetchNetworkReferenda(network.id),
                sleep(Constants.LOADING_STATE_TRANSITION_MIN_MS),
            ]);
        }

        for (const delegate of this.dataStore.getDelegates()) {
            delegate.votes = [];
            for (const delegation of delegate.delegations) {
                const network = this.dataStore
                    .getNetworks()
                    .find((network) => network.id == delegation.networkId)!;
                this.ui.setLoadingDescription(
                    `loading<br>${delegate.name}<br>${network.display} votes`,
                );
                await Promise.all([
                    this.dataStore.fetchNetworkDelegateVotes(
                        delegation.networkId,
                        delegation.delegateAccountId,
                    ),
                    sleep(Constants.LOADING_STATE_TRANSITION_MIN_MS),
                ]);
            }
        }
    }
}

export { App };
