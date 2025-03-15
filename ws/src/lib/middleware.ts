import * as jose from 'jose';

const JWKS = jose.createRemoteJWKSet(
  new URL('http://localhost:8000/api/auth/jwks'),
);

export const authenticate = async (token: string): Promise<boolean> => {
  if (!token) {
    return false; // Unauthorized
  }
  try {
    const { payload } = await jose.jwtVerify(token, JWKS, {
      issuer: 'http://localhost:8000',
    });
    // TODO: change behavior
    console.log(payload);
    return true; // Authenticated
  } catch (error) {
    console.log(error);
    return false; // Forbidden
  }
};
