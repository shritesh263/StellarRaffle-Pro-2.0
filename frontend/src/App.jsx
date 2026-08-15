import React, { useState, useEffect } from 'react';
import logoImg from './assets/logo.jpg';

import Wallet from './components/Wallet';
import Balance from './components/Balance';
import RaffleInfo from './components/RaffleInfo';
import BuyTicket from './components/BuyTicket';
import TierSelector from './components/TierSelector';
import WinnerHistory from './components/WinnerHistory';
import MetricsDashboard from './components/MetricsDashboard';
import Monitoring from './components/Monitoring';
import DataIndex from './components/DataIndex';

// Pro Expansion Components
import VaultTracker from './components/VaultTracker';
import CountdownTimer from './components/CountdownTimer';
import OddsCalculator from './components/OddsCalculator';
import MultiTicketPurchase from './components/MultiTicketPurchase';
import MyTicketsDashboard from './components/MyTicketsDashboard';
import DrawHistory from './components/DrawHistory';
import StreakBonus from './components/StreakBonus';
import Leaderboard from './components/Leaderboard';

import { useFreighter } from './hooks/useFreighter';

const TABS = [
  { id: 'play',       icon: '🎮', label: 'Play Now'    },
  { id: 'history',    icon: '🏆', label: 'Winners'     },
  { id: 'referral',   icon: '🤝', label: 'Referral'    },
  { id: 'metrics',    icon: '📊', label: 'Metrics'     },
  { id: 'monitoring', icon: '⚙️',  label: 'Monitor'    },
  { id: 'indexing',   icon: '🗃️',  label: 'Index'      },
  { id: 'features',   icon: '🚀', label: 'Pro'         },
];

function App() {
  const {
    publicKey: pubKey,
    isFreighterInstalled,
    connecting,
    error: walletError,
    connectWallet,
    disconnectWallet,
  } = useFreighter();

  const [alert, setAlert]               = useState(null);
  const [refreshTrigger, setRefreshTrigger] = useState(0);
  const [activeTab, setActiveTab]       = useState('play');
  const [selectedTier, setSelectedTier] = useState('Bronze');
  const [history, setHistory]           = useState([]);

  useEffect(() => {
    if (walletError) setAlert({ type: 'error', message: walletError });
  }, [walletError]);

  useEffect(() => {
    if (!alert) return;
    const t = setTimeout(() => setAlert(null), 6000);
    return () => clearTimeout(t);
  }, [alert]);

  const handleTransactionSuccess = () => setRefreshTrigger(p => p + 1);

  const copyReferralLink = () => {
    navigator.clipboard.writeText(`${window.location.origin}?ref=${pubKey}`);
    setAlert({ type: 'success', message: 'Referral link copied to clipboard!' });
  };

  return (
    <div className="app-container">

      {/* ── HEADER ────────────────────────────────── */}
      <header className="site-header">
        <div className="logo-block">
          <img src={logoImg} alt="StellarRaffle Pro Logo" className="logo-img" />
          <div className="logo-text">
            <span className="logo-name">
              Stellar<span>Raffle</span>&nbsp;<span style={{ fontSize: '0.75em', fontWeight: 700, color: 'var(--cyan)', WebkitTextFillColor: 'var(--cyan)' }}>PRO</span>
            </span>
            <span className="logo-tagline">Decentralized Lottery on Soroban</span>
          </div>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
          <div className="header-badge">
            <div className="live-indicator" />
            Testnet Live
          </div>
          {/* inline compact wallet in header */}
          {pubKey ? (
            <div style={{
              display: 'flex', alignItems: 'center', gap: '0.5rem',
              background: 'hsla(210,40%,96%,0.04)',
              border: '1px solid var(--border-bright)',
              borderRadius: 'var(--r-sm)',
              padding: '0.45rem 0.9rem',
              fontSize: '0.8rem', color: 'var(--text-secondary)', fontFamily: "'JetBrains Mono', monospace",
            }}>
              <div className="live-indicator" />
              {pubKey.slice(0, 6)}…{pubKey.slice(-6)}
            </div>
          ) : null}
        </div>
      </header>

      {/* ── GLOBAL ALERT ────────────────────────── */}
      {alert && (
        <div className={`notification notif-${alert.type}`}>
          <span style={{ fontSize: '1.1rem' }}>{alert.type === 'success' ? '✨' : '⚠️'}</span>
          {alert.message}
        </div>
      )}

      {/* ── HERO SECTION ─────────────────────────── */}
      <section className="hero-section">
        <div className="hero-eyebrow">
          <span>⚡</span> Powered by Stellar Soroban
        </div>
        <h1 className="hero-title">
          Win Big with <span className="gradient-word">On-Chain</span><br />
          Provably Fair Raffles
        </h1>
        <p className="hero-subtitle">
          Buy tickets, earn streak bonuses, track the vault live &amp; claim your prize —
          all governed by transparent smart contracts on Stellar.
        </p>
      </section>

      {/* ── TABS ─────────────────────────────────── */}
      <div className="tabs-container">
        <nav className="tabs" role="tablist">
          {TABS.map(tab => (
            <button
              key={tab.id}
              role="tab"
              aria-selected={activeTab === tab.id}
              className={`tab-btn${activeTab === tab.id ? ' active' : ''}`}
              onClick={() => setActiveTab(tab.id)}
            >
              <span>{tab.icon}</span>
              <span>{tab.label}</span>
            </button>
          ))}
        </nav>
      </div>

      {/* ── MAIN DASHBOARD ───────────────────────── */}
      <div className="dash-grid">

        {/* LEFT: Content Area */}
        <main className="main-content" role="tabpanel">

          {/* ── PLAY TAB ── */}
          {activeTab === 'play' && (
            <div className="animate-in">
              <div className="glass-card" style={{ padding: '2.5rem' }}>
                <div className="section-header">
                  <div>
                    <h2 className="section-title">Choose Your Tier</h2>
                    <p className="section-subtitle">Select a ticket tier to enter the next draw</p>
                  </div>
                  <div style={{
                    padding: '0.4rem 1rem',
                    borderRadius: 'var(--r-sm)',
                    background: 'hsla(243,75%,59%,0.12)',
                    border: '1px solid var(--border-glow)',
                    fontSize: '0.78rem',
                    fontWeight: 800,
                    color: 'var(--indigo)',
                    letterSpacing: '0.06em',
                  }}>TESTNET LIVE</div>
                </div>

                <TierSelector selectedTier={selectedTier} setSelectedTier={setSelectedTier} />

                {pubKey ? (
                  <BuyTicket
                    tier={selectedTier}
                    setAlert={setAlert}
                    onSuccess={handleTransactionSuccess}
                  />
                ) : (
                  <div className="connect-prompt" style={{ marginTop: '2rem' }}>
                    <p style={{ fontSize: '1.5rem', marginBottom: '0.75rem' }}>🔐</p>
                    <p style={{ fontWeight: 600, color: 'var(--text-secondary)', marginBottom: '0.4rem' }}>
                      Wallet Authorization Required
                    </p>
                    <p>Connect your Freighter wallet to participate in the raffle.</p>
                  </div>
                )}
              </div>
            </div>
          )}

          {/* ── WINNERS TAB ── */}
          {activeTab === 'history' && (
            <div className="animate-in">
              <WinnerHistory history={history} />
            </div>
          )}

          {/* ── REFERRAL TAB ── */}
          {activeTab === 'referral' && (
            <div className="glass-card animate-in" style={{ padding: '2.5rem' }}>
              <h2 className="section-title" style={{ marginBottom: '0.5rem' }}>Referral Program</h2>
              <p className="subtitle" style={{ marginBottom: '2rem' }}>
                Earn <strong style={{ color: 'var(--emerald)' }}>1% instant XLM</strong> for every ticket
                your friends buy. Rewards distributed automatically on-chain.
              </p>

              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '1rem', marginBottom: '2rem' }}>
                {[
                  { label: 'Your Bonus', value: '1% Reward', color: 'var(--emerald)' },
                  { label: 'Payout Speed', value: 'Instant',  color: 'var(--cyan)'    },
                ].map(item => (
                  <div key={item.label} className="glass-card" style={{ padding: '1.25rem' }}>
                    <div className="stat-label">{item.label}</div>
                    <div style={{ fontSize: '1.5rem', fontWeight: 800, color: item.color }}>{item.value}</div>
                  </div>
                ))}
              </div>

              {pubKey ? (
                <>
                  <div className="stat-label">Your Personal Link</div>
                  <div className="referral-link-box">
                    <div className="referral-link-input">
                      {window.location.origin}?ref={pubKey}
                    </div>
                    <button
                      onClick={copyReferralLink}
                      className="btn-primary"
                      style={{ width: 'auto', padding: '0.7rem 1.5rem', borderRadius: 'var(--r-sm)' }}
                    >
                      Copy
                    </button>
                  </div>
                </>
              ) : (
                <div className="connect-prompt">
                  <p style={{ color: 'var(--indigo)', fontWeight: 700 }}>
                    Connect your wallet to generate your referral link.
                  </p>
                </div>
              )}
            </div>
          )}

          {activeTab === 'metrics'    && <div className="animate-in"><MetricsDashboard /></div>}
          {activeTab === 'monitoring' && <div className="animate-in"><Monitoring /></div>}
          {activeTab === 'indexing'   && <div className="animate-in"><DataIndex /></div>}

          {/* ── PRO FEATURES TAB ── */}
          {activeTab === 'features' && (
            <div className="animate-in" style={{ display: 'flex', flexDirection: 'column', gap: '1.75rem' }}>

              <div className="pro-features-header">
                <div className="pro-badge">✦ Pro Expansion</div>
                <h2 className="section-title">Advanced Soroban Features</h2>
                <p className="section-subtitle">
                  Live vault tracking, countdown timer, odds engine, bulk purchase, portfolio &amp; global leaderboard
                </p>
              </div>

              {/* Streak Bonus — only visible when connected */}
              <StreakBonus />

              {/* Vault Tracker — full width */}
              <VaultTracker />

              {/* Countdown + Odds — side by side */}
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '1.5rem' }}>
                <CountdownTimer />
                <OddsCalculator />
              </div>

              {/* Bulk Purchase */}
              <MultiTicketPurchase />

              {/* My Tickets + Draw History */}
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(320px, 1fr))', gap: '1.5rem' }}>
                <MyTicketsDashboard />
                <DrawHistory />
              </div>

              {/* Leaderboard */}
              <Leaderboard />
            </div>
          )}
        </main>

        {/* RIGHT: Sidebar */}
        <aside style={{ display: 'flex', flexDirection: 'column', gap: '1.5rem' }}>

          {/* Wallet Card */}
          <div className="glass-card wallet-card">
            <div className="stat-label" style={{ marginBottom: '1rem' }}>
              <span>🔐</span> Secure Account
            </div>
            <Wallet setAlert={setAlert} />
          </div>

          {/* Balance */}
          <Balance refreshTrigger={refreshTrigger} />

          {/* Raffle Info */}
          <RaffleInfo refreshTrigger={refreshTrigger} setHistory={setHistory} />

          {/* Network Info */}
          <div className="glass-card" style={{ padding: '1.5rem' }}>
            <div className="stat-label" style={{ marginBottom: '1rem' }}>
              <span>🌐</span> Network
            </div>
            {[
              { label: 'Chain',    value: 'Stellar Testnet'  },
              { label: 'VM',       value: 'Soroban WASM'     },
              { label: 'Finality', value: '~5 sec'           },
              { label: 'Fee',      value: '~0.001 XLM'       },
            ].map(row => (
              <div key={row.label} style={{
                display: 'flex', justifyContent: 'space-between',
                alignItems: 'center', paddingBottom: '0.65rem',
                marginBottom: '0.65rem', borderBottom: '1px solid hsla(210,40%,96%,0.05)',
              }}>
                <span style={{ fontSize: '0.82rem', color: 'var(--text-muted)' }}>{row.label}</span>
                <span style={{ fontSize: '0.82rem', fontWeight: 600, color: 'var(--text-secondary)' }}>{row.value}</span>
              </div>
            ))}
          </div>

        </aside>
      </div>

      {/* ── FOOTER ───────────────────────────────── */}
      <footer style={{
        marginTop: '5rem',
        paddingTop: '2rem',
        borderTop: '1px solid var(--border-bright)',
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
        flexWrap: 'wrap',
        gap: '1rem',
      }}>
        <div className="logo-block">
          <img src={logoImg} alt="Logo" className="logo-img" style={{ width: 32, height: 32 }} />
          <span style={{ fontSize: '0.9rem', fontWeight: 700, color: 'var(--text-secondary)' }}>
            StellarRaffle <span style={{ color: 'var(--cyan)' }}>Pro</span>
          </span>
        </div>
        <p style={{ fontSize: '0.8rem', color: 'var(--text-dim)' }}>
          © 2025 StellarRaffle Pro · Built on Stellar Soroban · All draws verifiably fair
        </p>
        <a
          href="https://github.com/shritesh263/StellarRaffle-Pro-2.0"
          target="_blank" rel="noreferrer"
          style={{ fontSize: '0.8rem', color: 'var(--text-muted)', textDecoration: 'none', fontWeight: 600 }}
        >
          GitHub →
        </a>
      </footer>

    </div>
  );
}

export default App;
