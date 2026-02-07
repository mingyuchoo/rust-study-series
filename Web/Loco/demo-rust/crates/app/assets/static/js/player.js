// [REQ-F004] MusicLoop Audio Player (2026-02-07)
// iframe 기반 embed 플레이어 + 무한 루프

const Player = {
    playlist: [],
    currentIndex: -1,
    isPlaying: false,

    init() {
        this.bar = document.getElementById('player-bar');
        this.titleEl = document.getElementById('player-title');
        this.artistEl = document.getElementById('player-artist');
        this.iframe = document.getElementById('player-iframe');
        this.toggleBtn = document.getElementById('player-toggle');
        this.loopCheck = document.getElementById('player-loop');

        if (this.toggleBtn) {
            this.toggleBtn.addEventListener('click', () => this.toggle());
        }
        const prevBtn = document.getElementById('player-prev');
        const nextBtn = document.getElementById('player-next');
        if (prevBtn) prevBtn.addEventListener('click', () => this.prev());
        if (nextBtn) nextBtn.addEventListener('click', () => this.next());

        // YouTube iframe API에서 영상 종료 이벤트를 감지하기 어려우므로
        // 주기적으로 embed URL 파라미터에 loop를 포함
    },

    play(id, title, artist, url) {
        // 기존 플레이리스트에 없으면 추가
        const existing = this.playlist.findIndex(t => t.id === id);
        if (existing === -1) {
            this.playlist.push({ id, title, artist, url });
            this.currentIndex = this.playlist.length - 1;
        } else {
            this.currentIndex = existing;
        }
        this.loadCurrent();
    },

    loadCurrent() {
        if (this.currentIndex < 0 || this.currentIndex >= this.playlist.length) return;
        const track = this.playlist[this.currentIndex];

        this.titleEl.textContent = track.title;
        this.artistEl.textContent = track.artist || '';
        this.bar.style.display = 'block';

        const embedUrl = this.toEmbedUrl(track.url);
        if (embedUrl) {
            this.iframe.src = embedUrl;
            this.iframe.style.display = 'block';
            this.isPlaying = true;
            this.toggleBtn.textContent = '\u23F8';
        }
    },

    toEmbedUrl(url) {
        // YouTube
        let match = url.match(/(?:youtube\.com\/watch\?v=|youtu\.be\/)([a-zA-Z0-9_-]{11})/);
        if (match) {
            return 'https://www.youtube.com/embed/' + match[1] + '?autoplay=1&loop=1&playlist=' + match[1];
        }
        // SoundCloud - embed API 사용
        if (url.includes('soundcloud.com')) {
            return 'https://w.soundcloud.com/player/?url=' + encodeURIComponent(url) + '&auto_play=true';
        }
        // 일반 오디오 URL은 그대로 반환 (iframe으로 재생 시도)
        return url;
    },

    toggle() {
        if (this.isPlaying) {
            this.iframe.src = '';
            this.isPlaying = false;
            this.toggleBtn.textContent = '\u25B6';
        } else {
            this.loadCurrent();
        }
    },

    next() {
        if (this.playlist.length === 0) return;
        this.currentIndex = (this.currentIndex + 1) % this.playlist.length;
        this.loadCurrent();
    },

    prev() {
        if (this.playlist.length === 0) return;
        this.currentIndex = (this.currentIndex - 1 + this.playlist.length) % this.playlist.length;
        this.loadCurrent();
    }
};

// 전역 함수로 노출
function playTrack(id, title, artist, url) {
    Player.play(id, title, artist, url);
}

// 페이지 로드 시 초기화
document.addEventListener('DOMContentLoaded', () => Player.init());

// 네비게이션 바에서 로그인 상태 반영
document.addEventListener('DOMContentLoaded', () => {
    const user = JSON.parse(localStorage.getItem('user') || 'null');
    if (user) {
        const navLinks = document.querySelector('.nav-links');
        if (navLinks) {
            navLinks.textContent = '';
            const links = [
                { href: '/', text: 'Home' },
                { href: '/my/tracks', text: 'My Tracks' },
                { href: '/tracks/new', text: 'Add Track' }
            ];
            links.forEach(l => {
                const a = document.createElement('a');
                a.href = l.href;
                a.textContent = l.text;
                navLinks.appendChild(a);
            });
            const nameSpan = document.createElement('span');
            nameSpan.className = 'nav-user';
            nameSpan.textContent = user.name;
            navLinks.appendChild(nameSpan);

            const logoutBtn = document.createElement('a');
            logoutBtn.href = '#';
            logoutBtn.textContent = 'Logout';
            logoutBtn.addEventListener('click', (e) => {
                e.preventDefault();
                localStorage.removeItem('token');
                localStorage.removeItem('user');
                window.location.href = '/';
            });
            navLinks.appendChild(logoutBtn);
        }
    }
});
