<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { activityFullDialogOpen } from '$lib/stores/dialogs.store';
</script>

<Dialog.Root bind:open={$activityFullDialogOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title class="font-bold">Info: Attività non disponibile</Dialog.Title>
			<Dialog.Description class="pt-3">
				L'attività selezionata non è più disponibile, ciò potrebbe essere dovuto al fatto che un
				altro utente l'ha selezionata e confermata prima di te.
			</Dialog.Description>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
