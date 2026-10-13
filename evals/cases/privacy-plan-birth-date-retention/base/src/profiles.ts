import { query } from "./db";

export interface Profile {
  userId: number;
  displayName: string;
}

/** Saves the profile form. */
export async function saveProfile(profile: Profile): Promise<void> {
  await query("UPDATE profiles SET display_name = $2 WHERE user_id = $1", [profile.userId, profile.displayName]);
}
