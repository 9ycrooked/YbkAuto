export type ResourceState = {
  completed: number;
  incomplete: number;
};

export type CompletionResult = {
  total: number;
  completed: number;
  failed: string[];
};

export type CourseSummary = {
  clazzCourseId: string;
  courseName: string;
  className: string | null;
  teacherName: string;
  courseStatus: string;
  createTime: string | null;
  resourceState: ResourceState;
};

export type DashboardState = {
  courses: CourseSummary[];
};

export type SessionUser = {
  userId: string;
  fullName: string;
  schoolName: string;
  studentNo: string;
  departmentName: string;
};

export type SessionState = {
  authenticated: boolean;
  rememberedUsername: string;
  user: SessionUser | null;
  dashboard: DashboardState | null;
};
