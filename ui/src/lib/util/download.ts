export const downloadFile = async (url: string, name?: string) => {
    try {
        const data = await (await fetch(url)).blob();

        const handle = await window.showSaveFilePicker({
            suggestedName: name,
        });

        const stream = await handle.createWritable();

        await stream.write(data);
        await stream.close();
    } catch (err) {
        if ((err as Error).name == "AbortError") return;

        const a = document.createElement("a");

        a.href = url;
        a.download = name ?? "";

        a.click();
    }
};
