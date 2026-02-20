<script lang="ts">
  import Header from '$lib/components/Header.svelte';
  import Footer from '$lib/components/Footer.svelte';
</script>

<Header />

<main>
  <!-- Hero Section -->
  <section class="hero">
    <div class="hero-content">
      <h1>Learn Without Limits</h1>
      <p>Start, switch, or advance your career with thousands of courses from expert instructors.</p>
      <div class="hero-search">
        <input type="text" placeholder="Search for anything" />
        <button>Search</button>
      </div>
    </div>
  </section>

  <!-- Course Grid Section -->
  <section class="courses">
    <h2>Featured Courses</h2>
    <div class="course-grid">
      <div class="course-card placeholder">Course 1</div>
      <div class="course-card placeholder">Course 2</div>
      <div class="course-card placeholder">Course 3</div>
      <div class="course-card placeholder">Course 4</div>
    </div>
  </section>
</main>

<Footer />

<style>
  :global(*, *::before, *::after) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    color: #1c1d1f;
  }

  .hero {
    background: linear-gradient(135deg, #1c1d1f 0%, #2d2f31 100%);
    color: white;
    padding: 4rem 2rem;
    text-align: center;
  }

  .hero-content {
    max-width: 680px;
    margin: 0 auto;
  }

  .hero h1 {
    font-size: 2.5rem;
    font-weight: 700;
    margin-bottom: 1rem;
  }

  .hero p {
    font-size: 1.125rem;
    margin-bottom: 2rem;
    opacity: 0.9;
  }

  .hero-search {
    display: flex;
    gap: 0.5rem;
    max-width: 500px;
    margin: 0 auto;
  }

  .hero-search input {
    flex: 1;
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
  }

  .hero-search button {
    padding: 0.75rem 1.5rem;
    background: #a435f0;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    font-weight: 600;
  }

  .hero-search button:hover {
    background: #8710d8;
  }

  .courses {
    max-width: 1200px;
    margin: 3rem auto;
    padding: 0 2rem;
  }

  .courses h2 {
    font-size: 1.75rem;
    margin-bottom: 1.5rem;
  }

  .course-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 1.5rem;
  }

  .course-card {
    border: 1px solid #d1d7dc;
    border-radius: 4px;
    padding: 1rem;
    min-height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f7f9fa;
    font-size: 0.9rem;
    color: #6a6f73;
  }
</style>
