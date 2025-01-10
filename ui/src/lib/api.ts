import { goto } from "$app/navigation";
import { persisted } from "svelte-persisted-store";
import { get } from "svelte/store";
import { ModHostClient } from "@modhost/api";

export const client = new ModHostClient();

const tokenStore = persisted<string | undefined>("auth-token", undefined);

export const setToken = (token?: string) => {
    tokenStore.set(token);
    checkClientToken();
};

export const getToken = () => get(tokenStore);
export const isLoggedIn = () => !!getToken();

export const checkClientToken = () => {
    if (isLoggedIn() && !client.hasToken()) client.setToken(getToken()!);
    else if (!isLoggedIn() && client.hasToken()) client.unsetToken();
};

export const beginLogin = (redirect: string) =>
    goto(
        redirect
            ? `/api/v1/auth/github/login?redirect_uri=${redirect}`
            : "/api/v1/auth/github/login",
    );
