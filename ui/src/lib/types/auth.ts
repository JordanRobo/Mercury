export interface JWTPayload {
	sub: {
		id: string;
		email: string;
		role: string;
	};
	exp: number;
	site: string;
}
