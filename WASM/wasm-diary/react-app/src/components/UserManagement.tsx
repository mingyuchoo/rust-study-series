import { useCallback, useEffect, useState } from "react";
import type { Role, UserView } from "../types/diary";

interface UserManagementProps {
  currentUserId: string;
  getAllUsers: () => UserView[];
  deleteUser: (id: string) => boolean;
  changeRole: (id: string, role: Role) => boolean;
}

export function UserManagement({
  currentUserId,
  getAllUsers,
  deleteUser,
  changeRole,
}: UserManagementProps) {
  const [users, setUsers] = useState<UserView[]>([]);

  const refreshUsers = useCallback(() => {
    setUsers(getAllUsers());
  }, [getAllUsers]);

  useEffect(() => {
    refreshUsers();
  }, [refreshUsers]);

  const handleChangeRole = useCallback(
    (id: string, currentRole: Role) => {
      const newRole: Role = currentRole === "Admin" ? "User" : "Admin";
      if (changeRole(id, newRole)) {
        refreshUsers();
      }
    },
    [changeRole, refreshUsers]
  );

  const handleDelete = useCallback(
    (id: string, username: string) => {
      if (window.confirm(`"${username}" 사용자를 정말 삭제하시겠습니까?`)) {
        if (deleteUser(id)) {
          refreshUsers();
        }
      }
    },
    [deleteUser, refreshUsers]
  );

  const formatDate = (iso: string) => {
    try {
      return new Date(iso).toLocaleDateString("ko-KR");
    } catch {
      return iso;
    }
  };

  return (
    <div className="diary-form user-management">
      <h2>사용자 관리</h2>
      <div className="user-table-wrapper">
        <table className="user-table">
          <thead>
            <tr>
              <th>사용자 ID</th>
              <th>닉네임</th>
              <th>역할</th>
              <th>가입일</th>
              <th>관리</th>
            </tr>
          </thead>
          <tbody>
            {users.map((u) => {
              const isSelf = u.id === currentUserId;
              return (
                <tr key={u.id}>
                  <td>{u.username}</td>
                  <td>{u.nickname || "-"}</td>
                  <td>
                    <span
                      className={`role-badge ${u.role === "Admin" ? "role-admin" : "role-user"}`}
                    >
                      {u.role === "Admin" ? "관리자" : "사용자"}
                    </span>
                  </td>
                  <td>{formatDate(u.created_at)}</td>
                  <td className="user-actions">
                    <button
                      className={`btn btn-sm ${u.role === "Admin" ? "btn-secondary" : "btn-edit"}`}
                      disabled={isSelf}
                      onClick={() => handleChangeRole(u.id, u.role)}
                      title={
                        isSelf
                          ? "자기 자신의 역할은 변경할 수 없습니다"
                          : undefined
                      }
                    >
                      {u.role === "Admin" ? "관리자 해제" : "관리자 부여"}
                    </button>
                    <button
                      className="btn btn-sm btn-delete"
                      disabled={isSelf}
                      onClick={() => handleDelete(u.id, u.username)}
                      title={
                        isSelf
                          ? "자기 자신은 삭제할 수 없습니다"
                          : undefined
                      }
                    >
                      삭제
                    </button>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
      <p className="user-count">총 {users.length}명의 사용자</p>
    </div>
  );
}
