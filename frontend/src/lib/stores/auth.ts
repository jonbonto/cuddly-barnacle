import { writable } from 'svelte/store';

export interface User {
  id: string;
  email: string;
  fullName: string;
}

export const user = writable<User | null>(null);
export const isAuthenticated = writable<boolean>(false);

user.subscribe((u) => {
  isAuthenticated.set(u !== null);
});
