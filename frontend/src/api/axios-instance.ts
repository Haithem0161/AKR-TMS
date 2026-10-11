import axios, { type AxiosError, type AxiosRequestConfig } from "axios";

// The generated paths are complete (`/api/v1/...`, `/healthz`), so the base URL stays empty.
export const AXIOS_INSTANCE = axios.create({ baseURL: "", withCredentials: true });

export const customInstance = <T>(config: AxiosRequestConfig): Promise<T> =>
  AXIOS_INSTANCE(config).then(({ data }) => data as T);

export type ErrorType<Error> = AxiosError<Error>;
export type BodyType<BodyData> = BodyData;
