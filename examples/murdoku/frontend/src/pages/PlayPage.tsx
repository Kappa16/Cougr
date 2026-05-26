import { useState, useCallback, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { ArrowLeft, LayoutPanelLeft } from 'lucide-react';
import { PuzzleGrid } from '../components/PuzzleGrid';
import { SuspectBar } from '../components/SuspectBar';
import { CluePanel } from '../components/CluePanel';
import { SolvedBanner } from '../components/SolvedBanner';
import { MOCK_PUZZLES } from '../data/mockData';
import type { GameState, Cell } from '../types';

function buildInitialBoard(size: number): Cell[] {
  return Array.from({ length: size * size }, () => ({ suspectId: null, status: 'empty' as const }));
}

function checkConflicts(board: Cell[], size: number): Cell[] {
  return board.map((cell, i) => {
    if (cell.suspectId === null) return { ...cell, status: 'empty' as const };
    const val = cell.suspectId;
    const row = Math.floor(i / size);
    const col = i % size;
    let conflict = false;
    for (let c = 0; c < size && !conflict; c++) {
      const j = row * size + c;
      if (j !== i && board[j].suspectId === val) conflict = true;
    }
    for (let r = 0; r < size && !conflict; r++) {
      const j = r * size + col;
      if (j !== i && board[j].suspectId === val) conflict = true;
    }
    return { ...cell, status: (conflict ? 'conflict' : 'filled') as Cell['status'] };
  });
}

function checkSolved(board: Cell[], solution: number[]): boolean {
  if (board.some(c => c.suspectId === null || c.status === 'conflict')) return false;
  return board.every((c, i) => c.suspectId === solution[i]);
}

export function PlayPage() {
  const { puzzleId } = useParams<{ puzzleId: string }>();
  const navigate = useNavigate();
  const puzzle = MOCK_PUZZLES.find(p => p.id === puzzleId);
  const [showBanner, setShowBanner] = useState(false);
  const [cluePanelOpen, setCluePanelOpen] = useState(true);

  const [gameState, setGameState] = useState<GameState>(() => {
    if (!puzzle) return { puzzleId: '', board: [], gridSize: 4, suspects: [], selectedSuspectId: null, placedSuspectIds: new Set(), isSolved: false, moveCount: 0 };
    return { puzzleId: puzzle.id, board: buildInitialBoard(puzzle.gridSize), gridSize: puzzle.gridSize, suspects: puzzle.suspects, selectedSuspectId: null, placedSuspectIds: new Set(), isSolved: false, moveCount: 0 };
  });

  useEffect(() => { if (gameState.isSolved) setShowBanner(true); }, [gameState.isSolved]);

  const handleCellClick = useCallback((idx: number) => {
    setGameState(prev => {
      if (!prev.selectedSuspectId || prev.isSolved) return prev;
      const newBoard = prev.board.map((c, i) => i === idx ? { ...c, suspectId: prev.selectedSuspectId } : c) as Cell[];
      const checked = checkConflicts(newBoard, prev.gridSize);
      const solved = puzzle ? checkSolved(checked, puzzle.solution) : false;
      const newPlaced = new Set(prev.placedSuspectIds);
      newPlaced.add(prev.selectedSuspectId!);
      return { ...prev, board: checked, placedSuspectIds: newPlaced, selectedSuspectId: null, moveCount: prev.moveCount + 1, isSolved: solved };
    });
  }, [puzzle]);

  const handleSelectSuspect = useCallback((id: number) => {
    setGameState(prev => ({ ...prev, selectedSuspectId: id }));
  }, []);

  if (!puzzle) {
    return (
      <main style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', padding: '2rem' }}>
        <div style={{ textAlign: 'center' }}>
          <h1 style={{ fontFamily: 'var(--font-serif)', color: 'var(--accent-red)', marginBottom: '1rem' }}>Case Not Found</h1>
          <p style={{ color: 'var(--noir-muted)', marginBottom: '1.5rem' }}>No puzzle matches this case ID.</p>
          <button className="btn-outline" onClick={() => navigate('/')}><ArrowLeft size={14} /> Back to Case Files</button>
        </div>
      </main>
    );
  }

  const selectedSuspect = gameState.suspects.find(s => s.id === gameState.selectedSuspectId);

  return (
    <main id="main-content" style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
      {/* Header bar */}
      <div style={{ padding: '0.875rem 1.25rem', borderBottom: '1px solid var(--noir-border)', display: 'flex', alignItems: 'center', gap: '1rem', flexWrap: 'wrap' }}>
        <button id="btn-back" className="btn-outline" onClick={() => navigate('/')} aria-label="Back to case files">
          <ArrowLeft size={14} aria-hidden="true" /> Back
        </button>
        <div style={{ flex: 1, minWidth: 0 }}>
          <h1 style={{ fontFamily: 'var(--font-serif)', fontSize: 'clamp(1rem, 3vw, 1.375rem)', fontWeight: 700, color: 'var(--noir-text)', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
            {puzzle.title}
          </h1>
          <p style={{ fontFamily: 'var(--font-mono)', fontSize: '0.6875rem', color: 'var(--noir-muted)', marginTop: 2 }}>
            {puzzle.gridSize}×{puzzle.gridSize} · {puzzle.difficulty} · {gameState.moveCount} moves
          </p>
        </div>
        <button id="btn-toggle-clues-mobile" className="btn-outline mobile-only" onClick={() => setCluePanelOpen(o => !o)}
          aria-expanded={cluePanelOpen} aria-label={cluePanelOpen ? 'Hide clues' : 'Show clues'}>
          <LayoutPanelLeft size={14} aria-hidden="true" />
          <span style={{ fontSize: '0.8125rem' }}>Clues</span>
        </button>
      </div>

      {/* Play area */}
      <div className="play-layout" style={{ flex: 1, display: 'grid', gap: '1rem', padding: '1.25rem', maxWidth: 1280, margin: '0 auto', width: '100%', alignItems: 'start' }}>
        {/* Clue panel */}
        <div className="play-clues" style={{ display: cluePanelOpen ? 'block' : 'none' }}>
          <CluePanel clues={puzzle.clues} suspects={puzzle.suspects} />
        </div>

        {/* Grid column */}
        <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '0.875rem' }}>
          {gameState.isSolved && (
            <p role="status" aria-live="polite" style={{ fontFamily: 'var(--font-serif)', fontStyle: 'italic', color: 'var(--accent-green)', fontSize: '0.9375rem', textAlign: 'center' }}>
              ✓ Case solved — all suspects correctly placed.
            </p>
          )}
          <PuzzleGrid gameState={gameState} onCellClick={handleCellClick} />
          <p aria-live="polite" style={{ fontFamily: 'var(--font-mono)', fontSize: '0.75rem', textAlign: 'center', color: selectedSuspect ? 'var(--accent-gold)' : 'var(--noir-muted)', minHeight: '1.2em' }}>
            {selectedSuspect
              ? `✦ Click a cell to place ${selectedSuspect.name}`
              : gameState.isSolved ? '' : 'Select a suspect, then click a cell.'}
          </p>
        </div>

        {/* Suspect bar */}
        <div className="play-suspects">
          <SuspectBar suspects={gameState.suspects} selectedSuspectId={gameState.selectedSuspectId}
            placedSuspectIds={gameState.placedSuspectIds} onSelect={handleSelectSuspect} orientation="vertical" />
        </div>
      </div>

      {showBanner && (
        <SolvedBanner puzzleTitle={puzzle.title} moveCount={gameState.moveCount}
          onDismiss={() => { setShowBanner(false); navigate('/'); }} />
      )}

      <style>{`
        .play-layout { grid-template-columns: min(280px,30%) 1fr min(220px,25%); }
        @media (max-width: 1023px) {
          .play-layout { grid-template-columns: 1fr !important; }
          .play-clues  { order: 3; }
          .play-suspects { order: 2; }
        }
        .mobile-only { display: none; }
        @media (max-width: 1023px) { .mobile-only { display: inline-flex; } }
        @media (min-width: 1024px) { .play-clues { display: block !important; } }
      `}</style>
    </main>
  );
}
