import { useCallback, useEffect, useRef, useState } from "react";
import { UserManager, Role as WasmRole } from "wasm-lib";
import type {
  LoginResult,
  RegistrationValidation,
  Role,
  UserView,
} from "../types/diary";

const USERS_STORAGE_KEY = "diary-users";
const SESSION_KEY = "diary-session";

const ROLE_TO_WASM: Record<Role, number> = {
  Admin: WasmRole.Admin,
  User: WasmRole.User,
};

export interface AuthUser {
  id: string;
  username: string;
  nickname?: string;
  role: Role;
}

export interface UseAuthReturn {
  user: AuthUser | null;
  login: (username: string, password: string) => LoginResult;
  register: (username: string, password: string, nickname?: string) => LoginResult;
  logout: () => void;
  getAllUsers: () => UserView[];
  deleteUser: (id: string) => boolean;
  changeRole: (id: string, role: Role) => boolean;
  validateRegistration: (
    username: string,
    password: string
  ) => RegistrationValidation;
  isAdmin: boolean;
}

export function useAuth(wasmReady: boolean): UseAuthReturn {
  const managerRef = useRef<UserManager | null>(null);
  const [user, setUser] = useState<AuthUser | null>(null);

  useEffect(() => {
    if (!wasmReady) return;

    const manager = new UserManager();
    const saved = localStorage.getItem(USERS_STORAGE_KEY);
    if (saved) {
      manager.load_from_json(saved);
    }
    managerRef.current = manager;

    // 세션 복원
    const session = localStorage.getItem(SESSION_KEY);
    if (session) {
      try {
        const parsed: AuthUser = JSON.parse(session);
        // 사용자가 아직 존재하는지 확인
        const role = manager.get_user_role(parsed.id);
        if (role) {
          setUser(parsed);
        } else {
          localStorage.removeItem(SESSION_KEY);
        }
      } catch {
        localStorage.removeItem(SESSION_KEY);
      }
    }

    // localStorage에 초기 admin 계정 저장
    localStorage.setItem(USERS_STORAGE_KEY, manager.save_to_json());

    return () => {
      manager.free();
      managerRef.current = null;
    };
  }, [wasmReady]);

  const persistUsers = useCallback(() => {
    const mgr = managerRef.current;
    if (!mgr) return;
    localStorage.setItem(USERS_STORAGE_KEY, mgr.save_to_json());
  }, []);

  const login = useCallback(
    (username: string, password: string): LoginResult => {
      const mgr = managerRef.current!;
      const result: LoginResult = JSON.parse(mgr.login(username, password));
      if (result.success && result.user_id && result.username && result.role) {
        const authUser: AuthUser = {
          id: result.user_id,
          username: result.username,
          nickname: result.nickname,
          role: result.role,
        };
        setUser(authUser);
        localStorage.setItem(SESSION_KEY, JSON.stringify(authUser));
      }
      return result;
    },
    []
  );

  const register = useCallback(
    (username: string, password: string, nickname?: string): LoginResult => {
      const mgr = managerRef.current!;
      const result: LoginResult = JSON.parse(
        mgr.register(username, password, ROLE_TO_WASM["User"], nickname)
      );
      if (result.success && result.user_id && result.username && result.role) {
        persistUsers();
        const authUser: AuthUser = {
          id: result.user_id,
          username: result.username,
          nickname: result.nickname,
          role: result.role,
        };
        setUser(authUser);
        localStorage.setItem(SESSION_KEY, JSON.stringify(authUser));
      }
      return result;
    },
    [persistUsers]
  );

  const logout = useCallback(() => {
    setUser(null);
    localStorage.removeItem(SESSION_KEY);
  }, []);

  const getAllUsers = useCallback((): UserView[] => {
    const mgr = managerRef.current!;
    return JSON.parse(mgr.get_all_users());
  }, []);

  const deleteUser = useCallback(
    (id: string): boolean => {
      const mgr = managerRef.current!;
      const result = mgr.delete_user(id);
      if (result) persistUsers();
      return result;
    },
    [persistUsers]
  );

  const changeRole = useCallback(
    (id: string, role: Role): boolean => {
      const mgr = managerRef.current!;
      const result = mgr.change_role(id, ROLE_TO_WASM[role]);
      if (result) persistUsers();
      return result;
    },
    [persistUsers]
  );

  const validateRegistration = useCallback(
    (username: string, password: string): RegistrationValidation => {
      const mgr = managerRef.current!;
      return JSON.parse(mgr.validate_registration(username, password));
    },
    []
  );

  return {
    user,
    login,
    register,
    logout,
    getAllUsers,
    deleteUser,
    changeRole,
    validateRegistration,
    isAdmin: user?.role === "Admin",
  };
}
