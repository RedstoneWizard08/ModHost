import { get } from "svelte/store";
import { _ } from "svelte-i18n";
import { gameVersions } from "./meta";

export const tryAggregateVersions = (vers: string[]): string[] => {
    const available = get(gameVersions) || [];

    if (!available) return vers;

    if (vers == available.map((v) => v.id) || vers.some((v) => v.includes("rd-")))
        return [get(_)("package.version.all")];

    const index = available.findIndex((v) => v.id == vers[0]);

    if (index == -1) return vers;

    const slice = available.slice(index, index + vers.length);
    let count = 0;

    for (let i = 0; i < vers.length; i++) {
        const real = vers[i];
        const avail = slice[i];

        if (real == avail.id) {
            count++;
        } else {
            break;
        }
    }

    if (count > 2) {
        const first = vers[0];
        const last = vers[count - 1];
        const remaining = vers.slice(count);

        if (remaining.length > 0) {
            return [`${first} - ${last}`, ...tryAggregateVersions(remaining)];
        } else {
            return [`${first} - ${last}`];
        }
    }

    const used = vers.slice(0, count);
    const remaining = vers.slice(count);

    if (remaining.length > 0) {
        return [...used, ...tryAggregateVersions(remaining)];
    } else {
        return used;
    }
};
