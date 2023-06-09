export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json }
  | Json[]

export interface Database {
  graphql_public: {
    Tables: {
      [_ in never]: never
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      graphql: {
        Args: {
          operationName?: string
          query?: string
          variables?: Json
          extensions?: Json
        }
        Returns: Json
      }
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
  public: {
    Tables: {
      model_collaborator_invitations: {
        Row: {
          created_at: string
          id: string
          invitee_email: string
          invitor: string
          model: string
          role: Database["public"]["Enums"]["role"]
        }
        Insert: {
          created_at?: string
          id: string
          invitee_email: string
          invitor: string
          model: string
          role: Database["public"]["Enums"]["role"]
        }
        Update: {
          created_at?: string
          id?: string
          invitee_email?: string
          invitor?: string
          model?: string
          role?: Database["public"]["Enums"]["role"]
        }
        Relationships: [
          {
            foreignKeyName: "model_collaborator_invitations_invitor_fkey"
            columns: ["invitor"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "model_collaborator_invitations_model_fkey"
            columns: ["model"]
            referencedRelation: "models"
            referencedColumns: ["id"]
          }
        ]
      }
      model_collaborators: {
        Row: {
          created_at: string
          grantor: string
          model: string
          role: Database["public"]["Enums"]["role"]
          user: string
        }
        Insert: {
          created_at?: string
          grantor: string
          model: string
          role: Database["public"]["Enums"]["role"]
          user: string
        }
        Update: {
          created_at?: string
          grantor?: string
          model?: string
          role?: Database["public"]["Enums"]["role"]
          user?: string
        }
        Relationships: [
          {
            foreignKeyName: "model_collaborators_grantor_fkey"
            columns: ["grantor"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "model_collaborators_model_fkey"
            columns: ["model"]
            referencedRelation: "models"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "model_collaborators_user_fkey"
            columns: ["user"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          }
        ]
      }
      model_events: {
        Row: {
          created_at: string
          data: Json | null
          id: string
          sequence: number
          subject: string
          type: Database["public"]["Enums"]["event_type"]
          user: string
        }
        Insert: {
          created_at?: string
          data?: Json | null
          id: string
          sequence?: number
          subject: string
          type: Database["public"]["Enums"]["event_type"]
          user: string
        }
        Update: {
          created_at?: string
          data?: Json | null
          id?: string
          sequence?: number
          subject?: string
          type?: Database["public"]["Enums"]["event_type"]
          user?: string
        }
        Relationships: [
          {
            foreignKeyName: "model_events_subject_fkey"
            columns: ["subject"]
            referencedRelation: "models"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "model_events_user_fkey"
            columns: ["user"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          }
        ]
      }
      model_patches: {
        Row: {
          created_at: string
          data: string
          id: string
          model: string
        }
        Insert: {
          created_at?: string
          data: string
          id: string
          model: string
        }
        Update: {
          created_at?: string
          data?: string
          id?: string
          model?: string
        }
        Relationships: [
          {
            foreignKeyName: "model_patches_model_fkey"
            columns: ["model"]
            referencedRelation: "models"
            referencedColumns: ["id"]
          }
        ]
      }
      models: {
        Row: {
          created_at: string
          creator: string
          description: string | null
          id: string
          name: string
        }
        Insert: {
          created_at?: string
          creator: string
          description?: string | null
          id: string
          name: string
        }
        Update: {
          created_at?: string
          creator?: string
          description?: string | null
          id?: string
          name?: string
        }
        Relationships: [
          {
            foreignKeyName: "models_creator_fkey"
            columns: ["creator"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          }
        ]
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      append_patch: {
        Args: {
          model_id: string
          patch_id: string
          patch_data: string
        }
        Returns: string
      }
      apply_client_changes: {
        Args: {
          changes: Json
        }
        Returns: undefined
      }
      create_model: {
        Args: {
          model_id: string
          model_name: string
          model_description: string
        }
        Returns: string
      }
      delete_model: {
        Args: {
          model_id: string
        }
        Returns: string
      }
      get_admin_or_better_models_for_auth_user: {
        Args: Record<PropertyKey, never>
        Returns: string[]
      }
      get_all_models_for_auth_user: {
        Args: Record<PropertyKey, never>
        Returns: string[]
      }
      get_editor_or_better_models_for_auth_user: {
        Args: Record<PropertyKey, never>
        Returns: string[]
      }
      get_models_owned_by_auth_user: {
        Args: Record<PropertyKey, never>
        Returns: string[]
      }
      grant_collaborator_role: {
        Args: {
          model_id: string
          grantee_id: string
          role: Database["public"]["Enums"]["role"]
        }
        Returns: string
      }
      model_events_since: {
        Args: {
          starting_event_id: string
        }
        Returns: {
          id: string
          type: Database["public"]["Enums"]["event_type"]
          subject: string
          user: string
          data: Json
          patch_data: string
        }[]
      }
      revoke_collaborator_role: {
        Args: {
          model_id: string
          revokee_id: string
        }
        Returns: string
      }
      snapshot_model: {
        Args: {
          model_id: string
          model_data: string
          as_of_event: string
        }
        Returns: string
      }
      update_model: {
        Args: {
          model_id: string
          model_name: string
          model_description: string
        }
        Returns: string
      }
    }
    Enums: {
      event_type:
        | "created"
        | "updated"
        | "patched"
        | "deleted"
        | "collaborator_invited"
        | "collaborator_role_granted"
        | "collaborator_role_revoked"
        | "snapshotted"
      role: "owner" | "admin" | "editor" | "viewer"
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
  storage: {
    Tables: {
      buckets: {
        Row: {
          allowed_mime_types: string[] | null
          avif_autodetection: boolean | null
          created_at: string | null
          file_size_limit: number | null
          id: string
          name: string
          owner: string | null
          public: boolean | null
          updated_at: string | null
        }
        Insert: {
          allowed_mime_types?: string[] | null
          avif_autodetection?: boolean | null
          created_at?: string | null
          file_size_limit?: number | null
          id: string
          name: string
          owner?: string | null
          public?: boolean | null
          updated_at?: string | null
        }
        Update: {
          allowed_mime_types?: string[] | null
          avif_autodetection?: boolean | null
          created_at?: string | null
          file_size_limit?: number | null
          id?: string
          name?: string
          owner?: string | null
          public?: boolean | null
          updated_at?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "buckets_owner_fkey"
            columns: ["owner"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          }
        ]
      }
      migrations: {
        Row: {
          executed_at: string | null
          hash: string
          id: number
          name: string
        }
        Insert: {
          executed_at?: string | null
          hash: string
          id: number
          name: string
        }
        Update: {
          executed_at?: string | null
          hash?: string
          id?: number
          name?: string
        }
        Relationships: []
      }
      objects: {
        Row: {
          bucket_id: string | null
          created_at: string | null
          id: string
          last_accessed_at: string | null
          metadata: Json | null
          name: string | null
          owner: string | null
          path_tokens: string[] | null
          updated_at: string | null
          version: string | null
        }
        Insert: {
          bucket_id?: string | null
          created_at?: string | null
          id?: string
          last_accessed_at?: string | null
          metadata?: Json | null
          name?: string | null
          owner?: string | null
          path_tokens?: string[] | null
          updated_at?: string | null
          version?: string | null
        }
        Update: {
          bucket_id?: string | null
          created_at?: string | null
          id?: string
          last_accessed_at?: string | null
          metadata?: Json | null
          name?: string | null
          owner?: string | null
          path_tokens?: string[] | null
          updated_at?: string | null
          version?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "objects_bucketId_fkey"
            columns: ["bucket_id"]
            referencedRelation: "buckets"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "objects_owner_fkey"
            columns: ["owner"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          }
        ]
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      can_insert_object: {
        Args: {
          bucketid: string
          name: string
          owner: string
          metadata: Json
        }
        Returns: undefined
      }
      extension: {
        Args: {
          name: string
        }
        Returns: string
      }
      filename: {
        Args: {
          name: string
        }
        Returns: string
      }
      foldername: {
        Args: {
          name: string
        }
        Returns: unknown
      }
      get_size_by_bucket: {
        Args: Record<PropertyKey, never>
        Returns: {
          size: number
          bucket_id: string
        }[]
      }
      search: {
        Args: {
          prefix: string
          bucketname: string
          limits?: number
          levels?: number
          offsets?: number
          search?: string
          sortcolumn?: string
          sortorder?: string
        }
        Returns: {
          name: string
          id: string
          updated_at: string
          created_at: string
          last_accessed_at: string
          metadata: Json
        }[]
      }
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}

