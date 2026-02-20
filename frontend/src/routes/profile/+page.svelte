<script lang="ts">
  import { user, isAuthenticated } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  onMount(() => {
    if (!$isAuthenticated) {
      goto('/login');
    }
  });
</script>

{#if $isAuthenticated && $user}
  <main>
    <div class="profile-container">
      <h1>My Profile</h1>
      <div class="profile-info">
        <div class="field">
          <span class="field-label">Full Name</span>
          <span class="field-value">{$user.fullName}</span>
        </div>
        <div class="field">
          <span class="field-label">Email</span>
          <span class="field-value">{$user.email}</span>
        </div>
        <div class="field">
          <span class="field-label">Role</span>
          <span class="field-value role-badge {$user.role}">{$user.role === 'instructor' ? 'Instructor' : 'Student'}</span>
        </div>
      </div>
    </div>
  </main>
{/if}

<style>
  main {
    display: flex;
    justify-content: center;
    padding: 3rem 2rem;
    background: #f7f9fa;
    min-height: calc(100vh - 56px);
  }

  .profile-container {
    background: #fff;
    border: 1px solid #d1d7dc;
    border-radius: 8px;
    padding: 2rem;
    width: 100%;
    max-width: 500px;
  }

  h1 {
    font-size: 1.5rem;
    margin-bottom: 1.5rem;
    color: #1c1d1f;
  }

  .profile-info {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .field-label {
    font-size: 0.85rem;
    color: #6a6f73;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .field-value {
    font-size: 1rem;
    color: #1c1d1f;
  }

  .role-badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 999px;
    font-size: 0.85rem;
    font-weight: 600;
    width: fit-content;
  }

  .role-badge.student {
    background: #e0f0ff;
    color: #0065cc;
  }

  .role-badge.instructor {
    background: #f3e8ff;
    color: #7c3aed;
  }
</style>
