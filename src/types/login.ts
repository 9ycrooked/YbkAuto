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
  clazz_course_id: string;
  course_name: string;
  class_name: string | null;
  teacher_name: string;
  course_status: string;
  create_time: string | null;
  resource_state: ResourceState;
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
