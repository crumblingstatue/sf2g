
set(INCROOT ${PROJECT_SOURCE_DIR}/include/SFML/System)
set(SRCROOT ${PROJECT_SOURCE_DIR}/src/SFML/System)

# all source files
set(SRC
    ${SRCROOT}/Clock.cpp
    ${INCROOT}/Clock.hpp
    ${SRCROOT}/Err.cpp
    ${INCROOT}/Err.hpp
    ${INCROOT}/Export.hpp
    ${SRCROOT}/Lock.cpp
    ${INCROOT}/Lock.hpp
    ${SRCROOT}/Mutex.cpp
    ${INCROOT}/Mutex.hpp
    ${INCROOT}/NativeActivity.hpp
    ${INCROOT}/NonCopyable.hpp
    ${SRCROOT}/Sleep.cpp
    ${INCROOT}/Sleep.hpp
    ${SRCROOT}/String.cpp
    ${INCROOT}/String.hpp
    ${INCROOT}/String.inl
    ${SRCROOT}/Thread.cpp
    ${INCROOT}/Thread.hpp
    ${INCROOT}/Thread.inl
    ${SRCROOT}/ThreadLocal.cpp
    ${INCROOT}/ThreadLocal.hpp
    ${INCROOT}/ThreadLocalPtr.hpp
    ${INCROOT}/ThreadLocalPtr.inl
    ${SRCROOT}/Time.cpp
    ${INCROOT}/Time.hpp
    ${INCROOT}/Utf.hpp
    ${INCROOT}/Utf.inl
    ${INCROOT}/Vector2.hpp
    ${INCROOT}/Vector2.inl
    ${INCROOT}/Vector3.hpp
    ${INCROOT}/Vector3.inl
)
source_group("" FILES ${SRC})

# add platform specific sources
if(SFML_OS_WINDOWS)
    set(PLATFORM_SRC
        ${SRCROOT}/Win32/ClockImpl.cpp
        ${SRCROOT}/Win32/ClockImpl.hpp
        ${SRCROOT}/Win32/MutexImpl.cpp
        ${SRCROOT}/Win32/MutexImpl.hpp
        ${SRCROOT}/Win32/SleepImpl.cpp
        ${SRCROOT}/Win32/SleepImpl.hpp
        ${SRCROOT}/Win32/ThreadImpl.cpp
        ${SRCROOT}/Win32/ThreadImpl.hpp
        ${SRCROOT}/Win32/ThreadLocalImpl.cpp
        ${SRCROOT}/Win32/ThreadLocalImpl.hpp
    )
    source_group("windows" FILES ${PLATFORM_SRC})
else()
    set(PLATFORM_SRC
        ${SRCROOT}/Unix/ClockImpl.cpp
        ${SRCROOT}/Unix/ClockImpl.hpp
        ${SRCROOT}/Unix/MutexImpl.cpp
        ${SRCROOT}/Unix/MutexImpl.hpp
        ${SRCROOT}/Unix/SleepImpl.cpp
        ${SRCROOT}/Unix/SleepImpl.hpp
        ${SRCROOT}/Unix/ThreadImpl.cpp
        ${SRCROOT}/Unix/ThreadImpl.hpp
        ${SRCROOT}/Unix/ThreadLocalImpl.cpp
        ${SRCROOT}/Unix/ThreadLocalImpl.hpp
    )

    if(SFML_OS_ANDROID)
        set(PLATFORM_SRC ${PLATFORM_SRC}
            ${SRCROOT}/Android/Activity.hpp
            ${SRCROOT}/Android/Activity.cpp
            ${SRCROOT}/Android/NativeActivity.cpp
        )
    endif()

    source_group("unix" FILES ${PLATFORM_SRC})
endif()

# define the sfml-system target
sfml_add_library(sfml-system
                 SOURCES ${SRC} ${PLATFORM_SRC})

if(SFML_OS_ANDROID)
    # glad sources
    target_include_directories(sfml-system PRIVATE "${PROJECT_SOURCE_DIR}/extlibs/headers/glad/include")
endif()

# setup dependencies
if(SFML_OS_LINUX OR SFML_OS_FREEBSD OR SFML_OS_MACOSX)
    target_link_libraries(sfml-system PRIVATE pthread)
endif()
if(SFML_OS_LINUX)
    target_link_libraries(sfml-system PRIVATE rt)
elseif(SFML_OS_WINDOWS)
    target_link_libraries(sfml-system PRIVATE winmm)
elseif(SFML_OS_ANDROID)
    target_link_libraries(sfml-system PRIVATE android log)
endif()
