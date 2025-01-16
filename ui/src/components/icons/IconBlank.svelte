<script lang="ts">
    interface Props {
        size?: number;
        class?: string;
        [key: string]: any;
    }

    const { size = 24, class: klass, ...rest }: Props = $props();
</script>

<svg {...rest} width={size} height={size} class={klass ?? ""} />
