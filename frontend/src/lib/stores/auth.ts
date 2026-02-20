import { writable } from 'svelte/store';

export interface User {
  id: string;
  email: string;
  fullName: string;
  role: 'student' | 'instructor';
}

export const user = writable<User | null>(null);
export const isAuthenticated = writable<boolean>(false);
export const authToken = writable<string | null>(null);

// Persist token in localStorage
if (typeof window !== 'undefined') {
  const storedToken = localStorage.getItem('auth_token');
  const storedUser = localStorage.getItem('auth_user');
  if (storedToken && storedUser) {
    authToken.set(storedToken);
    user.set(JSON.parse(storedUser));
  }
}

user.subscribe((u) => {
  isAuthenticated.set(u !== null);
  if (typeof window !== 'undefined') {
    if (u) {
      localStorage.setItem('auth_user', JSON.stringify(u));
    } else {
      localStorage.removeItem('auth_user');
      localStorage.removeItem('auth_token');
    }
  }
});

authToken.subscribe((t) => {
  if (typeof window !== 'undefined') {
    if (t) {
      localStorage.setItem('auth_token', t);
    } else {
      localStorage.removeItem('auth_token');
    }
  }
});

export function logout() {
  user.set(null);
  authToken.set(null);
}
