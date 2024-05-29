import express from 'express';
import path from 'path';

const app = express();
const port = process.env.PORT || 3000;

app.use(express.static(path.resolve(__dirname, '../../frontend/build')));

app.get('/api', (req, res) => {
  res.json({ message: "Hello from TS server!" });
});

app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
});