import express, { Request, Response } from 'express';

export const notes_router = express.Router()

notes_router.get('', (req: Request, res: Response) => {
    res.send("List all notes");
});

notes_router.post('', (req: Request, res: Response) => {
    res.send("Create a new note");
});

notes_router.get('/:noteId', (req: Request, res: Response) => {
    res.send(`Get a specific note ${req.params.noteId}`);
});
