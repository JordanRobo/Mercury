import axios from 'axios';
import type { AxiosInstance, AxiosRequestConfig } from 'axios';

const MERCURY_API_URL = Bun.env.MERCURY_API_URL || 'http://127.0.0.1:2323/api';

class MercuryClient {
    protected client: AxiosInstance;

    constructor(options: AxiosRequestConfig = {}) {
        this.client = axios.create({
            baseURL: MERCURY_API_URL,
            ...options,
        });
    }
}

export default MercuryClient;