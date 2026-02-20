<script lang="ts">
  import { goto } from '$app/navigation';
  import { user, authToken } from '$lib/stores/auth';
  import { env } from '$env/dynamic/public';

  let email = '';
  let password = '';
  let errorMessage = '';
  let loading = false;

  const apiUrl = env.PUBLIC_API_URL || 'http://localhost:8080';

  async function handleLogin(event: Event) {
    event.preventDefault();
    errorMessage = '';
    loading = true;
    try {
      const res = await fetch(`${apiUrl}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password })
      });
      const data = await res.json();
      if (!res.ok) {
        errorMessage = data.error || 'Login failed';
        return;
      }
      authToken.set(data.token);
      user.set({
        id: data.user.id,
        email: data.user.email,
        fullName: data.user.full_name,
        role: data.user.role
      });
      goto('/');
    } catch {
      errorMessage = 'An unexpected error occurred.';
    } finally {
      loading = false;
    }
  }
</script>

<main>
  <div class="auth-container">
    <h1>Log in to Learnify</h1>
    {#if errorMessage}
      <p class="error">{errorMessage}</p>
    {/if}
    <form on:submit={handleLogin}>
      <label for="email">Email</label>
      <input id="email" type="email" bind:value={email} required autocomplete="email" />

      <label for="password">Password</label>
      <input id="password" type="password" bind:value={password} required autocomplete="current-password" />

      <button type="submit" disabled={loading}>{loading ? 'Logging in…' : 'Log in'}</button>
    </form>
    <p class="switch-link">Don't have an account? <a href="/signup">Sign up</a></p>
  </div>
</main>

<style>
  main {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: calc(100vh - 56px);
    padding: 2rem;
    background: #f7f9fa;
  }

  .auth-container {
    background: #fff;
    border: 1px solid #d1d7dc;
    border-radius: 8px;
    padding: 2rem;
    width: 100%;
    max-width: 400px;
  }

  h1 {
    font-size: 1.5rem;
    margin-bottom: 1.5rem;
    color: #1c1d1f;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  label {
    font-size: 0.9rem;
    font-weight: 600;
    color: #1c1d1f;
  }

  input {
    padding: 0.6rem 0.75rem;
    border: 1px solid #d1d7dc;
    border-radius: 4px;
    font-size: 1rem;
    width: 100%;
  }

  input:focus {
    outline: none;
    border-color: #a435f0;
  }

  button[type='submit'] {
    margin-top: 0.5rem;
    padding: 0.75rem;
    background: #a435f0;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
  }

  button[type='submit']:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .error {
    color: #d92d20;
    font-size: 0.9rem;
    margin-bottom: 1rem;
  }

  .switch-link {
    margin-top: 1.25rem;
    font-size: 0.9rem;
    text-align: center;
    color: #6a6f73;
  }

  .switch-link a {
    color: #a435f0;
    text-decoration: none;
  }
</style>
