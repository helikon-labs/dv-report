import { DataStore, DataStoreDelegate } from '../data/data-store';
import { UI, UIDelegate } from '../ui/ui';
import { sleep } from '../util/async-util';
import { Constants } from '../util/constants';

class App {
    private readonly ui: UI;
    private readonly uiDelegate = <UIDelegate>{};
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
            let delegateVoteCounts = this.dataStore.getDelegateVoteCounts();
            this.ui.barChart(delegateVoteCounts);
        } catch (error) {
            alert(`Error while fetching initial data: ${error}. Please reload the page.`);
            return;
        }
        this.ui.unlock();
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

        for (let network of this.dataStore.getNetworks()) {
            this.ui.setLoadingDescription(`loading ${network.display} referenda`);
            await Promise.all([
                this.dataStore.fetchNetworkReferenda(network.id),
                sleep(Constants.LOADING_STATE_TRANSITION_MIN_MS),
            ]);
        }

        for (let delegate of this.dataStore.getDelegates()) {
            delegate.votes = [];
            for (let delegation of delegate.delegations) {
                let network = this.dataStore
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
