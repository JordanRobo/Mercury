export interface Sub {
	id: string;
	name: string;
	email: string;
	role: string;
}

export interface JWTPayload {
	sub: Sub;
	exp: number;
	site: string;
}
