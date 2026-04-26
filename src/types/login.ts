export type CheckinState = "open" | "closed" | "error";

export type OpenCheckinInfo = {
  checkinId: string;
  title: string;
  type: string;
};

export type ResourceState = {
  status: string;
  label: string;
};

export type CourseSummary = {
  clazzCourseId: string;
  courseName: string;
  className: string;
  teacherName: string;
  courseStatus: string;
  checkinState: CheckinState;
  openCheckin: OpenCheckinInfo | null;
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
