import { Session, SessionConfig } from 'neo4j-driver';
import { neo4jDriver } from './repository/neo4j-client';

/**
 * Neo4jConnection
 * 
 * Wrapper around Neo4j driver providing session management.
 * Used by all Neo4j repositories for database access.
 */
export class Neo4jConnection {
  private driver = neo4jDriver;

  /**
   * Get a Neo4j session
   * @param config Optional session configuration (defaultAccessMode: "READ" | "WRITE")
   * @returns Neo4j Session
   */
  getSession(config?: SessionConfig): Session {
    return this.driver.session(config);
  }

  /**
   * Verify connectivity to Neo4j
   */
  async verifyConnectivity(): Promise<boolean> {
    try {
      await this.driver.verifyConnectivity();
      return true;
    } catch (error) {
      console.error('Neo4j connectivity check failed:', error);
      return false;
    }
  }

  /**
   * Close the driver connection
   */
  async close(): Promise<void> {
    await this.driver.close();
  }
}

/**
 * Default connection instance
 */
export const defaultConnection = new Neo4jConnection();

