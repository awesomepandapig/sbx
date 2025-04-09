import * as jose from 'jose';

const JWKS_URL = 'http://localhost:8000/api/auth/jwks';
const ISSUER = 'http://localhost:8000';
const JWKS = jose.createRemoteJWKSet(new URL(JWKS_URL));

export const authenticate = async (
  jwt: string,
): Promise<{ authenticated: boolean; user_id?: string }> => {
  if (!jwt) {
    return { authenticated: false };
  }
  try {
    const { payload } = await jose.jwtVerify(jwt, JWKS, {
      issuer: ISSUER,
    });
    return { authenticated: true, user_id: payload.sub };
  } catch (error) {
    console.log(error);
    return { authenticated: false };
  }
};
