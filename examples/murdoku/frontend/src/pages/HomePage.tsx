import { useNavigate } from 'react-router-dom';
import { BookOpen, Plus } from 'lucide-react';
import { PuzzleCard } from '../components/PuzzleCard';
import { MOCK_SUMMARIES } from '../data/mockData';

export function HomePage() {
  const navigate = useNavigate();

  return (
    <main id="main-content" style={{ flex: 1, padding: '2rem 1.25rem', maxWidth: 1280, margin: '0 auto', width: '100%' }}>
      {/* Hero */}
      <section
        aria-labelledby="hero-heading"
        style={{
          textAlign: 'center',
          padding: '3rem 1rem 2.5rem',
          marginBottom: '2.5rem',
          borderBottom: '1px solid var(--noir-border)',
        }}
      >
        <div
          aria-hidden="true"
          style={{
            fontFamily: 'var(--font-mono)',
            fontSize: '0.6875rem',
            color: 'var(--accent-gold)',
            letterSpacing: '0.2em',
            textTransform: 'uppercase',
            marginBottom: '0.75rem',
          }}
        >
          ✦ On-Chain Murder Mystery ✦
        </div>

        <h1
          id="hero-heading"
          style={{
            fontFamily: 'var(--font-serif)',
            fontSize: 'clamp(2rem, 5vw, 3.5rem)',
            fontWeight: 700,
            color: 'var(--noir-text)',
            lineHeight: 1.1,
            marginBottom: '1rem',
          }}
        >
          The Case Files
        </h1>

        <p
          style={{
            fontFamily: 'var(--font-serif)',
            fontStyle: 'italic',
            color: 'var(--noir-muted)',
            fontSize: '1.0625rem',
            maxWidth: 520,
            margin: '0 auto 1.75rem',
            lineHeight: 1.6,
          }}
        >
          Every puzzle is a locked room. Every clue is a lie — or is it?
          Place the suspects. Solve the grid. The truth is on-chain.
        </p>

        <button
          id="btn-create-puzzle"
          className="btn-outline"
          onClick={() => navigate('/create')}
          aria-label="Create a new puzzle"
          style={{ borderColor: 'var(--accent-gold)', color: 'var(--accent-gold)' }}
        >
          <Plus size={14} aria-hidden="true" />
          Create a Case
        </button>
      </section>

      {/* Catalog heading */}
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          marginBottom: '1.25rem',
          gap: '1rem',
        }}
      >
        <h2
          style={{
            fontFamily: 'var(--font-serif)',
            fontSize: '1.375rem',
            color: 'var(--noir-text)',
            display: 'flex',
            alignItems: 'center',
            gap: '0.5rem',
          }}
        >
          <BookOpen size={18} style={{ color: 'var(--accent-gold)' }} aria-hidden="true" />
          Open Cases
        </h2>
        <span
          style={{
            fontFamily: 'var(--font-mono)',
            fontSize: '0.75rem',
            color: 'var(--noir-muted)',
          }}
        >
          {MOCK_SUMMARIES.length} available
        </span>
      </div>

      {/* Puzzle grid */}
      <div
        role="list"
        aria-label="Available puzzles"
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
          gap: '1rem',
        }}
      >
        {MOCK_SUMMARIES.map((puzzle) => (
          <div key={puzzle.id} role="listitem">
            <PuzzleCard
              puzzle={puzzle}
              onClick={(id) => navigate(`/play/${id}`)}
            />
          </div>
        ))}
      </div>
    </main>
  );
}
