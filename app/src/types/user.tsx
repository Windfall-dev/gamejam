export interface User {
  id: number | string;
  first_name: string;
  last_name?: string;
  username?: string;
  language_code?: string;
}