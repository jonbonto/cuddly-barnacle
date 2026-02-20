<footer>
  <div class="footer-content">
    <p>© {new Date().getFullYear()} Learnify, Inc.</p>
    <nav>
      <a href="/about">About</a>
      <a href="/careers">Careers</a>
      <a href="/privacy">Privacy</a>
      <a href="/terms">Terms</a>
    </nav>
  </div>
</footer>

<style>
  footer {
    background: #1c1d1f;
    color: #fff;
    padding: 2rem;
    margin-top: 4rem;
  }

  .footer-content {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
  }

  nav {
    display: flex;
    gap: 1.5rem;
  }

  nav a {
    color: #d1d7dc;
    text-decoration: none;
    font-size: 0.9rem;
  }

  nav a:hover {
    color: #fff;
  }
</style>
