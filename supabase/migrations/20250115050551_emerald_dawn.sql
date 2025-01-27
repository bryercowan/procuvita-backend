/*
  # Initial Schema Setup for LifeQuest AI

  1. New Tables
    - `users`
      - `id` (uuid, primary key) - Links to Supabase auth
      - `created_at` (timestamp)
      - `display_name` (text)
      - `level` (int) - Overall user level
      - `total_xp` (int) - Total XP across all categories
    
    - `goals`
      - `id` (uuid, primary key)
      - `user_id` (uuid) - References users
      - `category` (text) - e.g., Health, Career
      - `title` (text)
      - `description` (text)
      - `xp` (int) - Category-specific XP
      - `level` (int) - Category-specific level
      - `status` (text) - active/paused
      - `created_at` (timestamp)
      - `agent_id` (uuid) - References AI agents
    
    - `tasks`
      - `id` (uuid, primary key)
      - `goal_id` (uuid) - References goals
      - `title` (text)
      - `description` (text)
      - `xp_reward` (int)
      - `duration` (int) - in minutes
      - `priority` (text) - high/medium/low
      - `status` (text) - pending/in_progress/completed/delegated_to_ai
      - `ai_assignable` (boolean)
      - `created_at` (timestamp)
      - `completed_at` (timestamp)
    
    - `ai_agents`
      - `id` (uuid, primary key)
      - `name` (text)
      - `personality` (text) - stern/empathetic/balanced
      - `avatar_url` (text)
      - `specialty` (text)
      - `created_at` (timestamp)

  2. Security
    - Enable RLS on all tables
    - Add policies for authenticated users
*/

-- Users table
CREATE TABLE users (
  id uuid PRIMARY KEY REFERENCES auth.users(id),
  created_at timestamptz DEFAULT now(),
  display_name text,
  level int DEFAULT 1,
  total_xp int DEFAULT 0
);

ALTER TABLE users ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can read own data"
  ON users FOR SELECT
  TO authenticated
  USING (auth.uid() = id);

CREATE POLICY "Users can update own data"
  ON users FOR UPDATE
  TO authenticated
  USING (auth.uid() = id);

-- AI Agents table
CREATE TABLE ai_agents (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  name text NOT NULL,
  personality text CHECK (personality IN ('stern', 'empathetic', 'balanced')),
  avatar_url text,
  specialty text NOT NULL,
  created_at timestamptz DEFAULT now()
);

ALTER TABLE ai_agents ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Anyone can read agents"
  ON ai_agents FOR SELECT
  TO authenticated
  USING (true);

-- Goals table
CREATE TABLE goals (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid REFERENCES users(id) NOT NULL,
  category text NOT NULL,
  title text NOT NULL,
  description text,
  xp int DEFAULT 0,
  level int DEFAULT 1,
  status text DEFAULT 'active' CHECK (status IN ('active', 'paused')),
  created_at timestamptz DEFAULT now(),
  agent_id uuid REFERENCES ai_agents(id)
);

ALTER TABLE goals ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can CRUD own goals"
  ON goals FOR ALL
  TO authenticated
  USING (auth.uid() = user_id);

-- Tasks table
CREATE TABLE tasks (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  goal_id uuid REFERENCES goals(id) NOT NULL,
  title text NOT NULL,
  description text,
  xp_reward int DEFAULT 10,
  duration int, -- in minutes
  priority text CHECK (priority IN ('high', 'medium', 'low')),
  status text DEFAULT 'pending' 
    CHECK (status IN ('pending', 'in_progress', 'completed', 'delegated_to_ai')),
  ai_assignable boolean DEFAULT false,
  created_at timestamptz DEFAULT now(),
  completed_at timestamptz
);

ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can CRUD own tasks"
  ON tasks FOR ALL
  TO authenticated
  USING (
    EXISTS (
      SELECT 1 FROM goals
      WHERE goals.id = tasks.goal_id
      AND goals.user_id = auth.uid()
    )
  );

-- Insert default AI agents
INSERT INTO ai_agents (name, personality, avatar_url, specialty) VALUES
  ('Coach Alex', 'stern', 'https://images.unsplash.com/photo-1594824476967-48c8b964273f?auto=format&fit=crop&q=80&w=150&h=150', 'Health & Fitness'),
  ('Mentor Sarah', 'empathetic', 'https://images.unsplash.com/photo-1573496359142-b8d87734a5a2?auto=format&fit=crop&q=80&w=150&h=150', 'Career Development'),
  ('Guide Mike', 'balanced', 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?auto=format&fit=crop&q=80&w=150&h=150', 'Personal Development');