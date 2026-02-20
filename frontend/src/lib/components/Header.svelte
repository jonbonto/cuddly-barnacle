<script lang="ts">
  import { cartCount } from '$lib/stores/cart';
  import { user, isAuthenticated, logout } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  async function handleLogout() {
    logout();
    goto('/');
  }
</script>

<header>
  <nav>
    <a href="/" class="logo">Learnify</a>
    <div class="nav-links">
      <a href="/courses">Browse</a>
      {#if $user?.role === 'instructor'}
        <a href="/teach">Teach on Learnify</a>
      {/if}
    </div>
    <div class="nav-actions">
      <a href="/cart" class="cart">🛒 {$cartCount}</a>
      {#if $isAuthenticated}
        <span class="user-name">{$user?.fullName}</span>
        <a href="/profile" class="btn-outline">Profile</a>
        <button on:click={handleLogout} class="btn-fill">Log out</button>
      {:else}
        <a href="/login" class="btn-outline">Log in</a>
        <a href="/signup" class="btn-fill">Sign up</a>
      {/if}
    </div>
  </nav>
</header>

<style>
  header {
    background: #fff;
    border-bottom: 1px solid #d1d7dc;
    position: sticky;
    top: 0;
    z-index: 100;
  }

  nav {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
    height: 56px;
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }

  .logo {
    font-size: 1.5rem;
    font-weight: 700;
    color: #1c1d1f;
    text-decoration: none;
  }

  .nav-links {
    display: flex;
    gap: 1rem;
    flex: 1;
  }

  .nav-links a {
    color: #1c1d1f;
    text-decoration: none;
    font-size: 0.95rem;
  }

  .nav-links a:hover {
    color: #a435f0;
  }

  .nav-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .cart {
    color: #1c1d1f;
    text-decoration: none;
  }

  .user-name {
    font-size: 0.9rem;
    color: #1c1d1f;
    font-weight: 600;
  }

  .btn-outline {
    padding: 0.4rem 0.75rem;
    border: 1px solid #1c1d1f;
    border-radius: 4px;
    color: #1c1d1f;
    text-decoration: none;
    font-size: 0.9rem;
  }

  .btn-fill {
    padding: 0.4rem 0.75rem;
    background: #1c1d1f;
    color: white;
    border-radius: 4px;
    text-decoration: none;
    font-size: 0.9rem;
    border: none;
    cursor: pointer;
    font-family: inherit;
  }
</style>
