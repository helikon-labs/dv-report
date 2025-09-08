import { DataStore } from '../data/data-store';
import { UI, UIDelegate } from '../ui/ui';

class App {
    private readonly ui: UI;
    private readonly uiDelegate: UIDelegate = {
        onCohortSelectChanged: (value) => {
            this.dataStore.setSelectedCohortNumber(value);
            this.start();
        },
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
        onDelegateTypeSelectChanged: (value) => {
            if (value == 'all') {
                this.dataStore.selectDelegateTypes(this.dataStore.getDelegateTypes());
            } else {
                const delegateType = this.dataStore
                    .getDelegateTypes()
                    .find((t) => t.id.toString() == value)!;
                this.dataStore.selectDelegateTypes([delegateType]);
            }
            this.updateVoteCounts();
        },
    };

    private readonly dataStore: DataStore;

    constructor() {
        this.dataStore = new DataStore();
        this.ui = new UI(this.uiDelegate);
    }

    async start() {
        this.ui.cleanup();
        this.ui.lock();
        try {
            await this.initData();
            this.ui.initFilters(
                this.dataStore.getSelectedCohortNumber(),
                this.dataStore.getNetworks(),
                this.dataStore.getTracks(),
                this.dataStore.getReferendumStatuses(),
                this.dataStore.getDelegateTypes(),
            );
            this.ui.setTitleAndSubtitle(this.dataStore.getSelectedCohortNumber());
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
        this.dataStore.init();
        this.ui.setLoadingDescription('loading networks');
        await this.dataStore.fetchNetworks();
        this.ui.setLoadingDescription('loading tracks');
        await this.dataStore.fetchTracks();
        this.ui.setLoadingDescription('loading cohorts');
        await this.dataStore.fetchCohorts();
        this.ui.setLoadingDescription('loading statuses');
        await this.dataStore.fetchReferendumStatuses();
        this.ui.setLoadingDescription('loading delegate types');
        await this.dataStore.fetchDelegateTypes();
        this.ui.setLoadingDescription('loading delegates');
        await this.dataStore.fetchDelegates();

        for (const network of this.dataStore.getNetworks()) {
            this.ui.setLoadingDescription(`loading<br>${network.display} referenda`);
            await this.dataStore.fetchNetworkCohortReferenda(
                network.id,
                this.dataStore.getSelectedCohortNumber(),
            );
        }

        for (const delegate of this.dataStore.getDelegates()) {
            delegate.votes = [];
            for (const delegation of delegate.delegations) {
                if (delegation.cohortNumber != this.dataStore.getSelectedCohortNumber()) {
                    continue;
                }
                const network = this.dataStore
                    .getNetworks()
                    .find((network) => network.id == delegation.networkId)!;
                this.ui.setLoadingDescription(
                    `loading<br>${delegate.name}<br>${network.display} votes`,
                );
                await this.dataStore.fetchNetworkDelegateVotes(
                    delegation.networkId,
                    delegation.delegateAccountId,
                );
            }
        }
    }
}

export { App };
