import type { NextFunction, Request, Response } from "express";

/** A signed-in user of the accounting app, and the organization they belong to. */
export interface User {
  id: number;
  orgId: number;
}

/** Refuses a request without a valid session; sets `req.user` otherwise. */
export function requireUser(req: Request, res: Response, next: NextFunction) {
  const user = (req as Request & { user?: User }).user;
  if (!user) {
    res.status(401).json({ error: "sign in first" });
    return;
  }
  next();
}

/** The user `requireUser` let through. */
export function currentUser(req: Request): User {
  return (req as Request & { user: User }).user;
}
