import type { GameState, Suspect } from '../types';

interface PuzzleGridProps {
  gameState: GameState;
  onCellClick: (index: number) => void;
}

function getSuspectById(suspects: Suspect[], id: number | null): Suspect | undefined {
  if (id === null) return undefined;
  return suspects.find((s) => s.id === id);
}

function hexToRgba(hex: string, alpha: number): string {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return `rgba(${r},${g},${b},${alpha})`;
}

export function PuzzleGrid({ gameState, onCellClick }: PuzzleGridProps) {
  const { board, gridSize, suspects, isSolved } = gameState;

  return (
    <div
      role="grid"
      aria-label={`${gridSize}×${gridSize} puzzle grid`}
      aria-describedby="grid-instructions"
      style={{
        display: 'grid',
        gridTemplateColumns: `repeat(${gridSize}, 1fr)`,
        gap: 2,
        background: 'var(--noir-border)',
        border: '2px solid var(--noir-border)',
        borderRadius: 6,
        overflow: 'hidden',
        width: '100%',
        maxWidth: gridSize === 4 ? 360 : 440,
        margin: '0 auto',
      }}
    >
      <p id="grid-instructions" className="sr-only">
        Select a suspect from the suspect bar, then click a cell to place them. Conflicting cells are highlighted in red.
      </p>

      {board.map((cell, idx) => {
        const row = Math.floor(idx / gridSize);
        const col = idx % gridSize;
        const suspect = getSuspectById(suspects, cell.suspectId);
        const isConflict = cell.status === 'conflict';
        const isEmpty = cell.status === 'empty';

        let bgColor = 'var(--noir-surface)';
        if (isConflict) bgColor = 'rgba(192,57,43,0.22)';
        else if (isSolved) bgColor = 'rgba(39,174,96,0.18)';
        else if (suspect) bgColor = hexToRgba(suspect.color, 0.18);

        let borderColor = 'transparent';
        if (isConflict) borderColor = 'var(--accent-red)';

        return (
          <div
            key={idx}
            role="gridcell"
            id={`cell-${row}-${col}`}
            aria-label={`Row ${row + 1}, Column ${col + 1}: ${suspect ? suspect.name : 'empty'}${isConflict ? ', conflict' : ''}`}
            aria-selected={false}
            tabIndex={0}
            onClick={() => onCellClick(idx)}
            onKeyDown={(e) => { if (e.key === 'Enter' || e.key === ' ') onCellClick(idx); }}
            style={{
              background: bgColor,
              border: `2px solid ${borderColor}`,
              aspectRatio: '1',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              flexDirection: 'column',
              cursor: isEmpty ? 'pointer' : 'default',
              transition: isConflict
                ? 'background-color 0ms'
                : isSolved
                ? 'background-color 400ms ease-in-out'
                : 'background-color 150ms ease-in-out',
              position: 'relative',
              padding: '0.25rem',
            }}
          >
            {suspect && (
              <>
                {/* Suspect color accent dot */}
                <span
                  aria-hidden="true"
                  style={{
                    width: 8, height: 8,
                    borderRadius: '50%',
                    background: suspect.color,
                    marginBottom: 4,
                    boxShadow: `0 0 6px ${suspect.color}80`,
                    flexShrink: 0,
                  }}
                />
                <span
                  style={{
                    fontFamily: 'var(--font-mono)',
                    fontSize: gridSize === 5 ? '0.6rem' : '0.7rem',
                    fontWeight: 600,
                    color: isConflict ? 'var(--accent-red)' : 'var(--noir-text)',
                    textAlign: 'center',
                    lineHeight: 1.2,
                    wordBreak: 'break-word',
                    maxWidth: '100%',
                  }}
                >
                  {suspect.initials}
                </span>
              </>
            )}

            {/* Solved checkmark */}
            {isSolved && suspect && (
              <span
                aria-hidden="true"
                style={{
                  position: 'absolute',
                  top: 2, right: 4,
                  fontSize: '0.6rem',
                  color: 'var(--accent-green)',
                }}
              >
                ✓
              </span>
            )}
          </div>
        );
      })}
    </div>
  );
}
