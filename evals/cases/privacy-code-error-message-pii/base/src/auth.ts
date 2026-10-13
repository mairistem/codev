import type { NextFunction, Request, Response } from "express";

/** Lets the request through only when the session's user has `role`. */
export function requireRole(role: string) {
  return (req: Request, res: Response, next: NextFunction) => {
    const user = (req as Request & { user?: { roles: string[] } }).user;
    if (!user || !user.roles.includes(role)) {
      res.status(403).json({ error: "forbidden" });
      return;
    }
    next();
  };
}

/** The signed-in customer's id; the session middleware refused anonymous requests. */
export function currentCustomerId(req: Request): number {
  return (req as Request & { user: { customerId: number } }).user.customerId;
}
