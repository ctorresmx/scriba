import express from 'express';
import path from 'path';
import { notes_router } from './notes';

const app = express();
const port = process.env.PORT || 3000;

app.use(express.static(path.resolve(__dirname, '../../frontend/build')));

app.use('/api/notes', notes_router);

app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
});