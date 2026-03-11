const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
require('dotenv').config();

const app = express();
const port = process.env.PORT || 3000;

app.use(cors());
app.use(bodyParser.json());

// In-memory post storage (for MVP)
let posts = [
  { id: 1, author: 'george', message: 'Just started building zsh-chat! 🚀', timestamp: new Date() },
  { id: 2, author: 'alice', message: 'The terminal is the best social media platform.', timestamp: new Date() },
  { id: 3, author: 'bob', message: 'Is this thing on?', timestamp: new Date() }
];

// Routes
app.get('/api/posts', (req, res) => {
  res.json(posts);
});

app.post('/api/posts', (req, res) => {
  const { author, message } = req.body;
  if (!author || !message) {
    return res.status(400).json({ error: 'Author and message are required.' });
  }

  const newPost = {
    id: posts.length + 1,
    author,
    message,
    timestamp: new Date()
  };

  posts.unshift(newPost); // Add to the beginning of the feed
  res.status(201).json(newPost);
});

app.listen(port, () => {
  console.log(`zsh-chat server listening at http://localhost:${port}`);
});
