export const splitToRows = <T>(data: T[], rows: number): T[][] => {
    const out = [];
    const cols = Math.floor(data.length / rows);

    for (let i = 0; i < rows; i++) {
        const items = i == rows - 1 ? data.length - cols * i : cols;
        const start = Math.max(0, cols * i);

        out.push(data.slice(start, Math.min(start + items, data.length - 1)));
    }

    return out;
};

export const dedupe = <T>(arr: T[]): T[] => {
    const newArr: T[] = [];

    for (const item of arr) {
        if (!newArr.includes(item)) newArr.push(item);
    }

    return newArr;
};
