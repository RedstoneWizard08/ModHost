import { type ToastStore } from "@skeletonlabs/skeleton";

export const copyText = async (data: string, toasts: ToastStore) => {
    if (!window.isSecureContext) {
        console.error(
            "Clipboard action failed: app not running in secure context, see: https://developer.mozilla.org/en-US/docs/Web/API/Clipboard",
        );
        return {};
    }

    await copyToClipboard(data);

    toasts.trigger({
        message: "Copied to clipboard!",
        hideDismiss: true,
        timeout: 1000,
        background: "variant-filled-success",
    });
};

export const copyToClipboard = async (data: any, mimeType: string = "text/plain") => {
    if (navigator.clipboard.write) {
        await navigator.clipboard.write([
            new ClipboardItem({
                [mimeType]: new Blob([data], {
                    type: mimeType,
                }),
                ["text/plain"]: new Blob([data], {
                    type: "text/plain",
                }),
            }),
        ]);
    } else {
        // fallback since .writeText has wider browser support
        await new Promise((resolve) => {
            resolve(navigator.clipboard.writeText(String(data)));
        });
    }
};
