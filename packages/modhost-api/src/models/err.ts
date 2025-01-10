export class ErrorResponse extends Error {
    public constructor(cause: unknown) {
        super("An error occured while performing a request.", {
            cause,
        });
    }
}
