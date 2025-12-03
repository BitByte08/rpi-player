// server.ts
import express from 'express';
import ytSearch from 'yt-search';

const app = express();
const PORT = process.env.PORT || 3000;

app.get('/search', async (req, res) => {
    const query = req.query.q as string;
    if (!query) return res.status(400).json({ error: '검색어 필요' });

    try {
        const result = await ytSearch(query);
        const videos = result.videos.map(v => ({
            title: v.title,
            videoId: v.videoId,
            url: v.url,
            thumbnail: v.thumbnail,
            duration: v.duration
        }));
        res.json(videos);
    } catch (err) {
        console.error(err);
        res.status(500).json({ error: '검색 실패', details: String(err) });
    }
});

app.listen(PORT, () => console.log(`Server running on port ${PORT}`));
