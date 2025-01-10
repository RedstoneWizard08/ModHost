import { ErrorResponse } from "./models";

export const unwrap = <T>(response: T | ErrorResponse) => {
    // I can't make this a ternary and I'm sad :'(

    if (response instanceof ErrorResponse) {
        throw response;
    }

    return response;
};

export const unwrapOrNull = <T>(response: T | ErrorResponse) =>
    response instanceof ErrorResponse ? null : response;
