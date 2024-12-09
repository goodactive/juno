<script lang="ts">
	import { AccountIdentifier } from '@dfinity/ledger-icp';
	import type { Principal } from '@dfinity/principal';
	import { nonNullish, type TokenAmountV2 } from '@dfinity/utils';
	import { createEventDispatcher } from 'svelte';
	import { preventDefault } from 'svelte/legacy';
	import { getAccountIdentifier } from '$lib/api/icp-index.api';
	import Identifier from '$lib/components/ui/Identifier.svelte';
	import Value from '$lib/components/ui/Value.svelte';
	import { IC_TRANSACTION_FEE_ICP } from '$lib/constants/constants';
	import { sendTokens } from '$lib/services/tokens.services';
	import { authStore } from '$lib/stores/auth.store';
	import { wizardBusy } from '$lib/stores/busy.store';
	import { i18n } from '$lib/stores/i18n.store';
	import { formatE8sICP } from '$lib/utils/icp.utils';
	import { amountToICPToken } from '$lib/utils/token.utils';

	interface Props {
		missionControlId: Principal;
		balance: bigint | undefined;
		destination?: string;
		amount: string | undefined;
	}

	let {
		missionControlId,
		balance,
		destination = $bindable(''),
		amount = $bindable()
	}: Props = $props();

	let accountIdentifier: AccountIdentifier | undefined = $derived(
		getAccountIdentifier(missionControlId)
	);

	let token: TokenAmountV2 | undefined = $derived(amountToICPToken(amount));

	const dispatch = createEventDispatcher();

	const onSubmit = async () => {
		wizardBusy.start();

		dispatch('junoNext', 'in_progress');

		try {
			await sendTokens({
				missionControlId,
				identity: $authStore.identity,
				destination,
				token
			});

			dispatch('junoNext', 'ready');
		} catch (err: unknown) {
			dispatch('junoNext', 'error');
		}

		wizardBusy.stop();
	};
</script>

<h2>{$i18n.wallet.send}</h2>

<p>{$i18n.wallet.review_and_confirm}</p>

<form onsubmit={preventDefault(onSubmit)}>
	<div class="columns">
		<div class="card-container with-title from">
			<span class="title">{$i18n.wallet.tx_from}</span>

			<div class="content">
				<Value>
					{#snippet label()}
						{$i18n.wallet.wallet_id}
					{/snippet}
					<p class="identifier">
						<Identifier shorten={false} identifier={missionControlId.toText()} />
					</p>
				</Value>

				<Value>
					{#snippet label()}
						{$i18n.wallet.account_identifier}
					{/snippet}
					<p class="identifier">
						<Identifier identifier={accountIdentifier?.toHex() ?? ''} />
					</p>
				</Value>

				<Value>
					{#snippet label()}
						{$i18n.wallet.balance}
					{/snippet}
					<p>
						{#if nonNullish(balance)}<span>{formatE8sICP(balance)} <small>ICP</small></span>{/if}
					</p>
				</Value>
			</div>
		</div>

		<div class="card-container with-title">
			<span class="title">{$i18n.wallet.tx_to}</span>

			<div class="content">
				<Value>
					{#snippet label()}
						{$i18n.wallet.destination}
					{/snippet}
					<p class="identifier">
						<Identifier identifier={destination} />
					</p>
				</Value>
			</div>
		</div>

		<div class="card-container with-title sending">
			<span class="title">{$i18n.wallet.sending}</span>

			<div class="content">
				<Value>
					{#snippet label()}
						{$i18n.wallet.tx_amount}
					{/snippet}
					<p>
						{#if nonNullish(token)}<span>{formatE8sICP(token.toE8s())} <small>ICP</small></span
							>{/if}
					</p>
				</Value>

				<Value>
					{#snippet label()}
						{$i18n.wallet.fee}
					{/snippet}
					<p>
						<span>{formatE8sICP(IC_TRANSACTION_FEE_ICP)} <small>ICP</small></span>
					</p>
				</Value>
			</div>
		</div>
	</div>

	<div class="toolbar">
		<button type="button" onclick={() => dispatch('junoNext', 'form')}>{$i18n.core.back}</button>
		<button type="submit">{$i18n.core.confirm}</button>
	</div>
</form>

<style lang="scss">
	@use '../../styles/mixins/media';

	.columns {
		@include media.min-width(large) {
			display: grid;
			grid-template-columns: repeat(2, calc((100% - var(--padding-2x)) / 2));
			grid-column-gap: var(--padding-2x);
		}
	}

	.from {
		grid-row-start: 1;
		grid-row-end: 3;
	}

	.sending {
		grid-column-start: 2;
		grid-column-end: 3;
	}

	.identifier {
		margin: 0 0 var(--padding);
	}
</style>
