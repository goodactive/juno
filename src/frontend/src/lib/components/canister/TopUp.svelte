<script lang="ts">
	import { isNullish, nonNullish } from '@dfinity/utils';
	import { createEventDispatcher } from 'svelte';
	import IconPublish from '$lib/components/icons/IconPublish.svelte';
	import { getMissionControlBalance } from '$lib/services/balance.services';
	import { busy } from '$lib/stores/busy.store';
	import { i18n } from '$lib/stores/i18n.store';
	import { missionControlStore } from '$lib/stores/mission-control.store';
	import type { JunoModalDetail } from '$lib/types/modal';
	import { emit } from '$lib/utils/events.utils';

	interface Props {
		type: 'topup_satellite' | 'topup_mission_control' | 'topup_orbiter';
		detail?: JunoModalDetail | undefined;
	}

	let { type, detail = undefined }: Props = $props();

	const dispatch = createEventDispatcher();

	const topUp = async () => {
		dispatch('junoTopUp');

		busy.start();

		const { result, error } = await getMissionControlBalance($missionControlStore);

		busy.stop();

		if (nonNullish(error) || isNullish(result)) {
			return;
		}

		emit({
			message: 'junoModal',
			detail: {
				type,
				detail: {
					...(nonNullish(detail) && detail),
					missionControlBalance: {
						...result
					}
				}
			}
		});
	};
</script>

<button onclick={topUp} class="menu"><IconPublish /> {$i18n.canisters.top_up}</button>
