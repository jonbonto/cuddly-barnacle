import { writable } from 'svelte/store';

export interface CartItem {
  courseId: string;
  title: string;
  price: number;
}

export const cart = writable<CartItem[]>([]);
export const cartCount = writable<number>(0);

cart.subscribe((items) => {
  cartCount.set(items.length);
});
