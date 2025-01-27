-- Add goals and knowledge_base to actors
ALTER TABLE ai_agents
ADD COLUMN goals JSONB DEFAULT '[]',
ADD COLUMN knowledge_base TEXT;

-- Create interactions table
CREATE TABLE interactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES ai_agents(id) NOT NULL,
    interaction_data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

ALTER TABLE interactions ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can access their interactions"
  ON interactions FOR ALL
  TO authenticated
  USING (
    EXISTS (
      SELECT 1 FROM goals
      WHERE goals.agent_id = interactions.actor_id
      AND goals.user_id = auth.uid()
    )
  );

-- Create state table
CREATE TABLE actor_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES ai_agents(id) NOT NULL,
    state_data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

ALTER TABLE actor_states ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can manage their actor states"
  ON actor_states FOR ALL
  TO authenticated
  USING (
    EXISTS (
      SELECT 1 FROM goals
      WHERE goals.agent_id = actor_states.actor_id
      AND goals.user_id = auth.uid()
    )
  );

-- Create historical interactions table
CREATE TABLE historical_interactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES ai_agents(id) NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

ALTER TABLE historical_interactions ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view historical interactions"
  ON historical_interactions FOR ALL
  TO authenticated
  USING (
    EXISTS (
      SELECT 1 FROM goals
      WHERE goals.agent_id = historical_interactions.actor_id
      AND goals.user_id = auth.uid()
    )
  );

